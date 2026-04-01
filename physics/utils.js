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

export {
  Hash,
  vecAdd,
  vecCopy,
  vecDistSquared,
  vecDot,
  vecLengthSquared,
  vecScale,
  vecSetDiff,
};
