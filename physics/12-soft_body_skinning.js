import { Hash } from "utils.js";
import { dragonTetMesh, dragonVisMesh } from "./data";

var gThreeScene;
var gRenderer;
var gCamera;
var gCameraControl;
var gGrabber;
var gMouseDown = false;

var gPhysicsScene = {
  gravity: [0.0, -10.0, 0.0],
  dt: 1.0 / 60.0,
  numSubsteps: 10,
  paused: true,
  showTets: false,
  objects: [],
};

function onShowTets() {
  gPhysicsScene.showTets = !gPhysicsScene.showTets;
  for (var i = 0; i < gPhysicsScene.objects.length; i++) {
    gPhysicsScene.objects[i].tetMesh.visible = gPhysicsScene.showTets;
  }
}

function initPhysics() {
  var body = new SoftBody(dragonTetMesh, dragonVisMesh, gThreeScene);
  gPhysicsScene.objects.push(body);
  document.getElementById("numTets").innerHTML = body.numTets;
  document.getElementById("numTris").innerHTML =
    dragonVisMesh.triIds.length / 3;
  document.getElementById("numVerts").innerHTML =
    dragonVisMesh.verts.length / 3;
}

function simulate() {
  if (gPhysicsScene.paused) {
    return;
  }

  var sdt = gPhysicsScene.dt / gPhysicsScene.numSubsteps;

  for (var step = 0; step < gPhysicsScene.numSubsteps; step++) {
    for (var i = 0; i < gPhysicsScene.objects.length; i++) {
      gPhysicsScene.objects[i].preSolve(std, gPhysicsScene.gravity);
    }

    for (var i = 0; i < gPhysicsScene.objects.length; i++) {
      gPhysicsScene.objects[i].solve(sdt);
    }

    for (var i = 0; i < gPhysicsScene.objects.length; i++) {
      gPhysicsScene.objects[i].postSolve(sdt);
    }
  }

  for (var i = 0; i < gPhysicsScene.objects.length; i++) {
    gPhysicsScene.objects[i].endFrame();
  }

  gGrabber.increaseTime(gPhysicsScene.dt);
}

function initThreeScene() {
  gThreeScene = new THREE.Scene();

  // Lights
  gThreeScene.add(new THREE.AmbientLight(0x505050));
  gThreeScene.fog = new THREE.Fog(0x000000, 0, 15);

  var spotLight = new THREE.SpotLight(0xffffff);
  spotLight.angle = Math.PI / 5;
  spotLight.penumbra = 0.2;
  spotLight.position.set(2, 3, 3);
  spotLight.castShadow = true;
  spotLight.shadow.camera.near = 3;
  spotLight.shadow.camera.far = 3;
  spotLight.shadow.mapSize.width = 1024;
  spotLight.shadow.mapSize.height = 1024;
  gThreeScene.add(spotLight);

  var dirLight = new THREE.DirectionalLight(0x55505a, 1);
  dirLight.position.set(0, 3, 0);
  dirLight.castShadow = true;
  dirLight.shadow.camera.near = 1;
  dirLight.shadow.camera.far = 10;

  dirLight.shadow.camera.right = 1;
  dirLight.shadow.camera.left = -1;
  dirLight.shadow.camera.top = 1;
  dirLight.shadow.camera.bottom = -1;

  dirLight.shadow.mapSize.width = 1024;
  dirLight.shadow.mapSize.height = 1024;
  gThreeScene.add(dirLight);

  // geometry

  var ground = new THREE.Mesh(
    new THREE.PlaneBufferGeometry(20, 20, 1, 1),
    new THREE.MeshPhongMaterial({ color: 0xa0adaf, shininess: 150 }),
  );
}
