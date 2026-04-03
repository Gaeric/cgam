class Hash {
  constructor(spacing, maxNumObjects) {
    this.spacing = spacing;
    this.tableSize = 2 * maxNumObjects;
    this.cellStart = new Int32Array(this.tableSize + 1);
    this.cellEntries = new Int32Array(maxNumObjects);
    this.queryIds = new Int32Array(maxNumObjects);
    this.querySize = 0;
  }

  hashCoords(xi, yi, zi) {
    // fantasy function
    var h = (xi * 92837111) ^ (yi * 689287499) ^ (zi * 283923481);
    return Math.abs(h) % this.tableSize;
  }

  intCoord(coord) {
    return Math.floor(coord / this.spacing);
  }

  hashPos(pos, nr) {
    return this.hashCoords(
      this.intCoord(pos[3 * nr]),
      this.intCoord(pos[3 * nr + 1]),
      this.intCoord(pos[3 * nr + 2]),
    );
  }

  create(pos) {
    var numObjects = Math.min(pos.length / 3, this.cellEntries.length);

    // determine cell sizes
    this.cellStart.fill(0);
    this.cellEntries.fill(0);

    for (var i = 0; i < numObjects; i++) {
      var h = this.hashPos(pos, i);
      this.cellStart[h]++;
    }

    // determine cells starts
    var start = 0;
    for (var i = 0; i < this.tableSize; i++) {
      start += this.cellStart[i];
      this.cellStart[i] = start;
    }

    // guard
    this.cellStart[this.tableSize] = start;

    // fill in objects ids
    for (var i = 0; i < numObjects; i++) {
      var h = this.hashPos(pos, i);
      this.cellStart[h]--;
      this.cellEntries[this.cellStart[h]] = i;
    }
  }

  query(pos, nr, maxDist) {
    var x0 = this.intCoord(pos[3 * nr] - maxDist);
    var y0 = this.intCoord(pos[3 * nr + 1] - maxDist);
    var z0 = this.intCoord(pos[3 * nr + 2] - maxDist);

    var x1 = this.intCoord(pos[3 * nr] + maxDist);
    var y1 = this.intCoord(pos[3 * nr + 1] + maxDist);
    var z1 = this.intCoord(pos[3 * nr + 2] + maxDist);

    this.querySize = 0;

    for (var xi = x0; xi <= x1; xi++) {
      for (var yi = y0; yi <= y1; yi++) {
        for (var zi = z0; zi <= z1; zi++) {
          var h = this.hashCoords(xi, yi, zi);
          var start = this.cellStart[h];
          var end = this.cellStart[h + 1];

          for (var i = start; i < end; i++) {
            this.queryIds[this.querySize] = this.cellEntries[i];
            this.querySize++;
          }
        }
      }
    }
  }
}

class Grabber {
  constructor() {
    this.raycaster = new THREE.Raycaster();
    this.raycaster.layers.set(1);
    this.raycaster.params.Line.threshold = 0.1;
    this.physicsObject = null;
    this.distance = 0.0;
    this.prevPos = new THREE.Vector3();
    this.vel = new THREE.Vector3();
    this.time = 0.0;
  }

  increaseTime(dt) {
    this.time += dt;
  }
  updateRaycaster(x, y) {
    var rect = gRenderer.domElement.getBoundingClientRect();
    this.mousePos = new THREE.Vector2();
    this.mousePos.x = ((x - rect.left) / rect.width) * 2 - 1;
    this.mousePos.y = -((y - rect.top) / rect.height) * 2 + 1;
    this.raycaster.setFromCamera(this.mousePos, gCamera);
  }

  start(x, y) {
    this.physicsObject = null;
    this.updateRaycaster(x, y);
    var intersects = this.raycaster.intersectObjects(gThreeScene.children);
    if (intersects.length > 0) {
      var obj = intersects[0].object.userData;
      if (obj) {
        this.physicsObject = obj;
        this.distance = intersects[0].distance;
        var pos = this.raycaster.ray.origin.clone();
        pos.addScaleVector(this.raycaster.ray.direction, this.distance);
        this.physicsObject.startGrab(pos);
        this.prevPos.copy(pos);
        this.vel.set(0.0, 0.0, 0.0);
        this.time = 0.0;
        if (gPhysicsScene.paused) {
          run();
        }
      }
    }
  }

  move(x, y) {
    if (this.physicsObject) {
      this.updateRaycaster(x, y);
      var pos = this.raycaster.ray.origin.clone();
      pos.addScaleVector(this.raycaster.ray.direction, this.distance);

      this.vel.copy(pos);
      this.vel.sub(this.prevPos);
      if (this.time > 0.0) {
        this.vel.divideScalar(this.time);
      } else {
        vel.set(0.0, 0.0, 0.0);
      }
      this.prevPos.copy(pos);
      this.time = 0.0;
      this.physicsObject.moveGrabbed(pos, this.vel);
    }
  }

  end(x, y) {
    if (this.physicsObject) {
      this.physicsObject.endGrab(this.prevPos, this.vel);
      this.physicsObject = null;
    }
  }
}

class SoftBody {
  constructor(
    tetMesh,
    visMesh,
    scene,
    edgeCompliance = 0.0,
    volCompliance = 0.0,
  ) {
    // physics

    this.numParticles = tetMesh.verts.length / 3;
    this.numTets = tetMesh.tetIds.length / 4;
    this.pos = new Float32Array(tetMesh.verts);
    this.prevPos = tetMesh.verts.slice();
    this.vel = new Float32Array(3 * this.numParticles);

    this.tetIds = tetMesh.tetIds;
    this.edgeIds = tetMesh.edgeIds;
    this.restVol = new Float32Array(this.numTets);
    this.edgeLengths = new Float32Array(this.edgeIds.length / 2);
    this.invMass = new Float32Array(this.numParticles);

    this.edgeCompliance = edgeCompliance;
    this.volCompliance = volCompliance;

    this.temp = new Float32Array(4 * 3);
    this.grads = new Float32Array(4 * 3);

    this.grabId = -1;
    this.grabInvMass = 0.0;

    this.initPhysics();

    // visual tet mesh
    var geometry = new THREE.BufferGeometry();
    geometry.setAttribute("position", new THREE.BufferAttribute(this.pos, 3));
    geometry.setIndex(tetMesh.edgeIds);
    var lineMaterial = new THREE.LineBasicMaterial({
      color: 0xffffff,
      linewidth: 2,
    });
    this.tetMesh = new THREE.LineSegments(geometry, lineMaterial);
    this.tetMesh.visible = true;
    scene.add(this.tetMesh);
    this.tetMesh.visible = false;

    // visual embedded mesh
    this.numVisVerts = visMesh.verts.length / 3;
    this.skinningInfo = new Float32Array(4 * this.numVisVerts);
    this.computeSkinningInfo(visMesh.verts);

    geometry = new THREE.BufferGeometry();
    geometry.setAttribute(
      "position",
      new THREE.BufferAttribute(new Float32Array(3 * this.numVisVerts), 3),
    );
    geometry.setIndex(visMesh.triIds);
    var visMaterial = new THREE.Mesh(geometry, visMaterial);
    this.visMesh = new THREE.Mesh(geometry, visMaterial);
    this.visMesh.castShadow = true;
    this.visMesh.userData = this;
    this.visMesh.layers.enable(1);
    scene.add(this.visMesh);
    geometry.computeVertexNormals();
    this.updateVisMesh();

    this.volIdOrder = [
      [1, 3, 2],
      [0, 2, 3],
      [0, 3, 1],
      [0, 1, 2],
    ];
  }

  computeSkinningInfo(visVerts) {
    var hash = new Hash(0.05, this.numVisVerts);
    hash.create(visVerts);
    this.skinningInfo.fill(-1.0);

    var minDist = new Float32Array(this.numVisVerts);
    minDist.fill(Number.MAX_VALUE);
    var border = 0.05;

    var tetCenter = new Float32Array(3);
    var mat = new Float32Array(9);
    var bary = new Float32Array(4);

    for (var i = 0; i < this.numTets; i++) {
      // compute bounding sphere of tet
      tetCenter.fill(0.0);
      for (var j = 0; j < 4; j++) {
        vecAdd(tetCenter, 0, this.pos, this.tetIds[4 * i + j], 0.25);
      }

      var rMax = 0.0;
      for (var j = 0; j < 4; j++) {
        var r2 = vecDistSquared(tetCenter, 0, this.pos, this.tetIds[4 * i + j]);
        rMax = Math.max(rMax, Math.sqrt(r2));
      }

      rMax += border;
      hash.query(tetCenter, 0, rMax);
      if (hash.queryIds.length == 0) {
        continue;
      }

      var id0 = this.tetIds[4 * i];
      var id1 = this.tetIds[4 * i + 1];
      var id2 = this.tetIds[4 * i + 2];
      var id3 = this.tetIds[4 * i + 3];

      vecSetDiff(mat, 0, this.pos, id0, this.pos, id3);
      vecSetDiff(mat, 1, this.pos, id1, this.pos, id3);
      vecSetDiff(mat, 2, this.pos, id2, this.pos, id3);

      matSetInverse(mat);

      for (var j = 0; j < hash.queryIds.length; j++) {
        var id = hash.queryIds[j];
        // we already have skinning info
        if (minDist[id] <= 0.0) {
          continue;
        }

        if (vecDistSquared(visVerts, id, tetCenter, 0) > rMax * rMax) {
          continue;
        }

        // compute barycentric coords for candidate
        vecSetDiff(bary, 0, visVerts, id, this.pos, id3);
        matSetMult(mat, bary, 0, bary, 0);
        bary[3] = 1.0 - bary[0] - bary[1] - bary[2];

        var dist = 0.0;
        for (var k = 0; k < 4; k++) {
          dist = Math.max(dist, -bary[k]);
        }

        if (dist < minDist[id]) {
          minDist[id] = dist;
          this.skinningInfo[4 * id] = i;
          this.skinningInfo[4 * id + 1] = bary[0];
          this.skinningInfo[4 * id + 2] = bary[1];
          this.skinningInfo[4 * id + 3] = bary[2];
        }
      }
    }
  }

  getTetVolume(nr) {
    var id0 = this.tetIds[4 * nr];
    var id1 = this.tetIds[4 * nr + 1];
    var id2 = this.tetIds[4 * nr + 2];
    var id3 = this.tetIds[4 * nr + 3];
    vecSetDiff(this.temp, 0, this.pos, id1, this.pos, id0);
    vecSetDiff(this.temp, 1, this.pos, id2, this.pos, id0);
    vecSetDiff(this.temp, 2, this.pos, id3, this.pos, id0);
    vecSetCross(this.temp, 3, this.temp, 0, this.temp, 1);

    return vecDot(this.temp, 3, this.temp, 2) / 6.0;
  }

  initPhysics() {
    this.invMass.fill(0.0);
    this.restVol.fill(0.0);

    for (var i = 0; i < this.numTets; i++) {
      var vol = this.getTetVolume(i);
      this.restVol[i] = vol;
      var pInvMass = vol > 0.0 ? 1.0 / (vol / 4.0) : 0.0;
      this.invMass[this.tetIds[4 * i]] += pInvMass;
      this.invMass[this.tetIds[4 * i + 1]] += pInvMass;
      this.invMass[this.tetIds[4 * i + 2]] += pInvMass;
      this.invMass[this.tetIds[4 * i + 3]] += pInvMass;
    }

    for (var i = 0; i < this.edgeLengths.length; i++) {
      var id0 = this.edgeIds[2 * i];
      var id1 = this.edgeIds[2 * i + 1];
      this.edgeLengths[i] = Math.sqrt(
        vecDistSquared(this.pos.id0, this.pos, id1),
      );
    }
  }

  preSolve(dt, gravity) {
    for (var i = 0; i < this.numParticles; i++) {
      if (this.invMass[i] == 0.0) {
        continue;
      }
      vecAdd(this.vel, i, gravity, 0, dt);
      vecCopy(this.prevPos, i, this.pos, i);
      vecAdd(this.pos, i, this.vel, i, dt);
      var y = this.pos[3 * i + 1];
      if (y < 0.0) {
        vecCopy(this.pos, i, this.prevPos, i);
        this.pos[3 * i + 1] = 0.0;
      }
    }
  }

  slove(dt) {
    this.solveEdges(this.edgeCompliance, dt);
    this.solveVolumes(this.volCompliance, dt);
  }

  postSolve(dt) {
    for (var i = 0; i < this.numParticles; i++) {
      if (this.invMass[i] == 0.0) {
        continue;
      }
      vecSetDiff(this.vel, i, this.pos, i, this.prevPos, i, 1.0 / dt);
    }
  }

  solveEdges(compliance, dt) {
    var alpha = compliance / dt / dt;
    for (var i = 0; i < this.edgeLengths.length; i++) {
      var id0 = this.edgeIds[2 * i];
      var id1 = this.edgeIds[2 * i + 1];
      var w0 = this.invMass[id0];
      var w1 = this.invMass[id1];
      var w = w0 + w1;
      if (w == 0.0) {
        continue;
      }

      vecSetDiff(this.grads, 0, this.pos, id0, this.pos, id1);
      var len = Math.sqrt(vecLengthSquared(this.grads, 0));
      if (len == 0.0) {
        continue;
      }

      vecScale(this.grads, 0, 1.0 / len);
      var restLen = this.edgeLengths[i];
      var C = len - restLen;
      var s = -C / (w + alpha);
      vecAdd(this.pos, id0, this.grads, 0, s * w0);
      vecAdd(this.pos, id1, this.grads, 0, -s * w1);
    }
  }

  solveVolumes(compliance, dt) {
    var alpha = compliance / dt / dt;
    for (var i = 0; i < this.numTets; i++) {
      var w = 0.0;
      for (var j = 0; j < 4; j++) {
        var id0 = this.tetIds[4 * i + this.volIdOrder[j][0]];
        var id1 = this.tetIds[4 * i + this.volIdOrder[j][1]];
        var id2 = this.tetIds[4 * i + this.volIdOrder[j][2]];

        vecSetDiff(this.temp, 0, this.pos, id1, this.pos, id0);
        vecSetDiff(this.temp, 1, this.pos, id2, this.pos, id0);
        vecSetCross(this.grads, j, this.temp, this.temp, 1);
        vecScale(this.grads, j, 1.0 / 6.0);

        w +=
          this.invMass[this.tetIds[4 * i + j]] *
          vecLengthSquared(this.grads, j);
      }

      if (w == 0.0) {
        continue;
      }

      var vol = this.getTetVolume(i);
      var restVol = this.restVol[i];
      var C = vol - restVol;
      var s = -C / (w + alpha);

      for (var j = 0; j < 4; j++) {
        var id = this.tetIds[4 * i + j];
        vecAdd(this.pos, id, this.grads, j, s * this.invMass[id]);
      }
    }
  }

  endFrame() {
    this.updateTetMesh();
    this.updateVisMesh();
  }

  updateTetMesh() {
    const positions = this.tetMesh.geometry.attributes.position.array;
    for (let i = 0; i < this.pos.length; i++) {
      positions[i] = this.pos[i];
    }
    this.tetMesh.geometry.attributes.position.needsUpdate = true;
    this.tetMesh.geometry.computeBoundingSphere();
  }

  updateVisMesh() {
    const positions = this.visMesh.geometry.attributes.position.array;
    var nr = 0;
    for (let i = 0; i < this.numVisVerts; i++) {
      var tetNr = this.skinningInfo[nr++] * 4;
      if (tetNr < 0) {
        nr += 3;
        continue;
      }

      var b0 = this.skinningInfo[nr++];
      var b1 = this.skinningInfo[nr++];
      var b2 = this.skinningInfo[nr++];
      var b3 = 1.0 - b0 - b1 - b2;

      var id0 = this.tetIds[tetNr++];
      var id1 = this.tetIds[tetNr++];
      var id2 = this.tetIds[tetNr++];
      var id3 = this.tetIds[tetNr++];
      vecSetZero(positions, i);
      vecAdd(positions, i, this.pos, id0, b0);
      vecAdd(positions, i, this.pos, id1, b1);
      vecAdd(positions, i, this.pos, id2, b2);
      vecAdd(positions, i, this.pos, id3, b3);
    }

    this.visMesh.geometry.computeVertexNormals();
    this.visMesh.geometry.attributes.position.needsUpdate = true;
    this.visMesh.geometry.computeBoundingSphere();
  }

  squash() {
    for (var i = 0; i < this.numParticles; i++) {
      this.pos[3 * i + 1] = 0.5;
    }
    this.endFrame();
  }

  startGrab(pos) {
    var p = [pos.x, pos.y, pos.z];
    var minD2 = Number.MAX_VALUE;
    this.grabId = -1;
    for (let i = 0; i < this.numParticles; i++) {
      var d2 = vecDistSquared(p, 0, this.pos, i);
      if (d2 < minD2) {
        minD2 = d2;
        this.grabId = i;
      }
    }

    if (this.grabId >= 0) {
      this.grabInvMass = this.invMass[this.grabId];
      this.invMass[this.grabId] = 0.0;
      vecCopy(this.pos, this.grabId, p, 0);
    }
  }

  moveGrabbed(pos, vel) {
    if (this.grabId >= 0) {
      var p = [pos.x, pos.y, pos.z];
      vecCopy(this.pos, this.grabId, p, 0);
    }
  }

  endGrab(pos, vel) {
    if (this.grabId >= 0) {
      this.invMass[this.grabId] = this.grabInvMass;
      var v = [vel.x, vel.y, vel.z];
      vecCopy(this.vel, this.grabId, v, 0);
    }
    this.grabId = -1;
  }
}

function vecScale(a, anr, scale) {
  anr *= 3;
  a[anr++] *= scale;
  a[anr++] *= scale;
  a[anr] *= scale;
}

function vecCopy(a, anr, b, bnr) {
  anr *= 3;
  bnr *= 3;
  a[anr++] = b[bnr++];
  a[anr++] = b[bnr++];
  a[anr] = b[bnr];
}

function vecAdd(a, anr, b, bnr, scale = 1.0) {
  anr *= 3;
  bnr *= 3;

  a[anr++] += b[bnr++] * scale;
  a[anr++] += b[bnr++] * scale;
  a[anr] += b[bnr] * scale;
}

function vecSetDiff(dst, dnr, a, anr, b, bnr, scale = 1.0) {
  dnr *= 3;
  anr *= 3;
  bnr *= 3;
  dst[dnr++] = (a[anr++] - b[bnr++]) * scale;
  dst[dnr++] = (a[anr++] - b[bnr++]) * scale;
  dst[dnr] = (a[anr] - b[bnr]) * scale;
}

function vecLengthSquared(a, anr) {
  anr *= 3;
  let a0 = a[anr],
    a1 = a[anr + 1],
    a2 = a[anr + 2];

  return a0 * a0 + a1 * a1 + a2 * a2;
}

function vecDistSquared(a, anr, b, bnr) {
  anr *= 3;
  bnr *= 3;
  let a0 = a[anr] - b[bnr];
  let a1 = a[anr + 1] - b[bnr + 1];
  let a2 = a[anr + 2] - b[bnr + 2];

  return a0 * a0 + a1 * a1 + a2 * a2;
}

function vecDot(a, anr, b, bnr) {
  anr *= 3;
  bnr *= 3;
  return a[anr] * b[bnr] + a[anr + 1] * b[bnr + 1] + a[anr + 2] * b[bnr + 2];
}

function vecSetCross(a, anr, b, bnr, c, cnr) {
  anr *= 3;
  bnr *= 3;
  cnr *= 3;
  a[anr++] = b[bnr + 1] * c[cnr + 2] - b[bnr + 2] * c[cnr + 1];
  a[anr++] = b[bnr + 2] * c[cnr + 0] - b[bnr + 0] * c[cnr + 2];
  a[anr] = b[bnr + 0] * c[cnr + 1] - b[bnr + 1] * c[cnr + 0];
}

function vecSetZero(a, anr) {
  anr *= 3;
  a[anr++] = 0.0;
  a[anr++] = 0.0;
  a[anr] = 0.0;
}

function matSetMult(A, a, anr, b, bnr) {
  bnr *= 3;
  var bx = b[bnr++];
  var by = b[bnr++];
  var bz = b[bnr];

  vecSetZero(a, anr);
  vecAdd(a, anr, A, 0, bx);
  vecAdd(a, anr, A, 1, by);
  vecAdd(a, anr, A, 2, bz);
}

function matGetDeterminant(A) {
  let a11 = A[0];
  let a12 = A[3];
  let a13 = A[6];
  let a21 = A[1];
  let a22 = A[4];
  let a23 = A[7];
  let a31 = A[2];
  let a32 = A[5];
  let a33 = A[8];

  return (
    a11 * a22 * a33 +
    a12 * a23 * a31 +
    a13 * a21 * a32 -
    a13 * a22 * a31 -
    a12 * a21 * a33 -
    a11 * a23 * a32
  );
}

function matSetInverse(A) {
  let det = matGetDeterminant(A);
  if (det == 0.0) {
    for (let i = 0; i < 9; i++) {
      A[i] = 0;
      return;
    }
  }

  let invDet = 1.0 / det;
  let a11 = A[0];
  let a21 = A[1];
  let a31 = A[2];

  let a12 = A[3];
  let a22 = A[4];
  let a32 = A[5];

  let a13 = A[6];
  let a23 = A[7];
  let a33 = A[8];

  A[0] = (a22 * a33 - a23 * a32) * invDet;
  A[3] = -(a12 * a33 - a13 * a32) * invDet;
  A[6] = (a12 * a23 - a13 * a22) * invDet;
  A[1] = -(a21 * a33 - a23 * a31) * invDet;
  A[4] = (a11 * a33 - a13 * a31) * invDet;
  A[7] = -(a11 * a23 - a13 * a21) * invDet;
  A[2] = (a21 * a32 - a22 * a31) * invDet;
  A[5] = -(a11 * a32 - a12 * a31) * invDet;
  A[8] = (a11 * a22 - a12 * a21) * invDet;
}

export {
  Hash,
  vecAdd,
  vecCopy,
  vecDistSquared,
  vecDot,
  vecLengthSquared,
  vecScale,
  vecSetDiff,
  vecSetCross,
  vecSetZero,
  matSetInverse,
  matGetDeterminant,
  matSetMult,
};
