import * as THREE from "three";
import init, { Simulation } from "../pkg/fish_schooling_simulator.js";
import "./styles.css";

const canvas2d = document.querySelector("#simulation-2d");
const context = canvas2d.getContext("2d");
const canvas3d = document.querySelector("#simulation-3d");
const speedButtons = [...document.querySelectorAll(".speed-button")];
const viewButtons = [...document.querySelectorAll(".view-button")];
const fishEatenValue = document.querySelector("#fish-eaten");
const fishEatenRateValue = document.querySelector("#fish-eaten-rate");
const elapsedTimeValue = document.querySelector("#elapsed-time");

await init();

const simulation = new Simulation();
const world = {
  width: simulation.world_width(),
  height: simulation.world_height(),
  depth: simulation.world_depth(),
};
const threeView = createThreeView();

let simulationSpeed = 1;
let viewMode = "2d";
let previousTime = performance.now();

function setSimulationSpeed(nextSpeed) {
  simulationSpeed = nextSpeed;

  for (const button of speedButtons) {
    const isActive = Number(button.dataset.speed) === simulationSpeed;
    button.classList.toggle("is-active", isActive);
    button.setAttribute("aria-pressed", String(isActive));
  }
}

function setViewMode(nextViewMode) {
  simulation.set_dimensions(nextViewMode === "3d" ? 3 : 2);
  viewMode = nextViewMode;

  for (const button of viewButtons) {
    const isActive = button.dataset.viewMode === viewMode;
    button.classList.toggle("is-active", isActive);
    button.setAttribute("aria-pressed", String(isActive));
  }

  canvas2d.classList.toggle("is-hidden", viewMode !== "2d");
  canvas3d.classList.toggle("is-hidden", viewMode !== "3d");
  resizeCanvases();
}

for (const button of speedButtons) {
  button.addEventListener("click", () => {
    setSimulationSpeed(Number(button.dataset.speed));
  });
}

for (const button of viewButtons) {
  button.addEventListener("click", () => {
    setViewMode(button.dataset.viewMode);
  });
}

function formatElapsedTime(totalSeconds) {
  const wholeSeconds = Math.floor(totalSeconds);
  const minutes = Math.floor(wholeSeconds / 60);
  const seconds = String(wholeSeconds % 60).padStart(2, "0");
  return `${minutes}:${seconds}`;
}

function resizeCanvases() {
  resizeCanvas2d();
  threeView.resize();
}

function resizeCanvas2d() {
  const pixelRatio = window.devicePixelRatio || 1;
  const { width, height } = canvas2d.getBoundingClientRect();
  canvas2d.width = Math.max(1, Math.floor(width * pixelRatio));
  canvas2d.height = Math.max(1, Math.floor(height * pixelRatio));
  context.setTransform(pixelRatio, 0, 0, pixelRatio, 0, 0);
}

function worldTransform() {
  const viewportWidth = canvas2d.clientWidth;
  const viewportHeight = canvas2d.clientHeight;
  const scale = Math.min(
    viewportWidth / world.width,
    viewportHeight / world.height,
  );
  const offsetX = (viewportWidth - world.width * scale) * 0.5;
  const offsetY = (viewportHeight - world.height * scale) * 0.5;

  return { scale, offsetX, offsetY };
}

function screenPoint(x, y, transform) {
  return {
    x: transform.offsetX + x * transform.scale,
    y: transform.offsetY + y * transform.scale,
  };
}

function drawFish2d(x, y, transform) {
  const point = screenPoint(x, y, transform);
  const radius = Math.max(2.2, 3.8 * transform.scale);
  context.beginPath();
  context.arc(point.x, point.y, radius, 0, Math.PI * 2);
  context.fill();
}

function drawShark2d(x, y, headingX, headingY, transform) {
  const point = screenPoint(x, y, transform);
  const size = Math.max(8, 16 * transform.scale);

  context.save();
  context.translate(point.x, point.y);
  context.rotate(Math.atan2(headingY, headingX));
  context.beginPath();
  context.moveTo(size, 0);
  context.lineTo(-size * 0.72, -size * 0.58);
  context.lineTo(-size * 0.42, 0);
  context.lineTo(-size * 0.72, size * 0.58);
  context.closePath();
  context.fill();
  context.restore();
}

function draw2d(state) {
  const transform = worldTransform();
  const viewportWidth = canvas2d.clientWidth;
  const viewportHeight = canvas2d.clientHeight;

  context.clearRect(0, 0, viewportWidth, viewportHeight);
  context.fillStyle = "#071923";
  context.fillRect(0, 0, viewportWidth, viewportHeight);

  context.strokeStyle = "rgba(155, 216, 217, 0.22)";
  context.lineWidth = 1;
  context.strokeRect(
    transform.offsetX,
    transform.offsetY,
    world.width * transform.scale,
    world.height * transform.scale,
  );

  const [sharkX, sharkY] = state.sharkPosition;
  const [headingX, headingY] = state.sharkHeading;
  const detectionRadius = simulation.shark_detection_radius() * transform.scale;
  const sharkPoint = screenPoint(sharkX, sharkY, transform);
  context.beginPath();
  context.arc(
    sharkPoint.x,
    sharkPoint.y,
    detectionRadius,
    0,
    Math.PI * 2,
  );
  context.fillStyle = "rgba(248, 113, 113, 0.08)";
  context.fill();

  if (state.sharkTarget.length === 2) {
    const targetPoint = screenPoint(
      state.sharkTarget[0],
      state.sharkTarget[1],
      transform,
    );
    const targetRadius = Math.max(7, 11 * transform.scale);
    const targetDeltaX = targetPoint.x - sharkPoint.x;
    const targetDeltaY = targetPoint.y - sharkPoint.y;
    const targetDistance = Math.hypot(targetDeltaX, targetDeltaY);
    const lineEnd =
      targetDistance > targetRadius
        ? {
            x: targetPoint.x - (targetDeltaX / targetDistance) * targetRadius,
            y: targetPoint.y - (targetDeltaY / targetDistance) * targetRadius,
          }
        : targetPoint;

    context.strokeStyle = "rgba(248, 113, 113, 0.48)";
    context.lineWidth = 1.5;
    context.beginPath();
    context.moveTo(sharkPoint.x, sharkPoint.y);
    context.lineTo(lineEnd.x, lineEnd.y);
    context.stroke();

    context.strokeStyle = "rgba(248, 113, 113, 0.82)";
    context.lineWidth = 2;
    context.beginPath();
    context.arc(targetPoint.x, targetPoint.y, targetRadius, 0, Math.PI * 2);
    context.stroke();
  }

  context.fillStyle = "#5eead4";
  for (let index = 0; index < state.fishPositions.length; index += 2) {
    drawFish2d(
      state.fishPositions[index],
      state.fishPositions[index + 1],
      transform,
    );
  }

  context.fillStyle = "#f87171";
  drawShark2d(sharkX, sharkY, headingX, headingY, transform);
}

function createThreeView() {
  const renderer = new THREE.WebGLRenderer({
    canvas: canvas3d,
    antialias: true,
  });
  renderer.setClearColor(0x071923, 1);

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x071923);

  const camera = new THREE.PerspectiveCamera(48, 1, 1, 3000);
  const cameraTarget = new THREE.Vector3(0, 0, 0);
  let cameraYaw = -0.35;
  let cameraPitch = 0.34;
  let cameraDistance = Math.max(world.width, world.height, world.depth) * 1.08;

  scene.add(new THREE.AmbientLight(0x9bd8d9, 0.55));

  const keyLight = new THREE.DirectionalLight(0xffffff, 1.25);
  keyLight.position.set(world.width * 0.25, world.height * 0.8, world.depth * 0.6);
  scene.add(keyLight);

  const boundsGeometry = new THREE.BoxGeometry(world.width, world.height, world.depth);
  const bounds = new THREE.LineSegments(
    new THREE.EdgesGeometry(boundsGeometry),
    new THREE.LineBasicMaterial({
      color: 0x9bd8d9,
      transparent: true,
      opacity: 0.26,
    }),
  );
  scene.add(bounds);

  const fishMesh = new THREE.InstancedMesh(
    new THREE.SphereGeometry(5.2, 12, 8),
    new THREE.MeshStandardMaterial({
      color: 0x5eead4,
      roughness: 0.62,
      metalness: 0.05,
    }),
    Math.max(1, simulation.fish_count()),
  );
  scene.add(fishMesh);

  const sharkGeometry = new THREE.ConeGeometry(14, 42, 3);
  sharkGeometry.rotateZ(-Math.PI / 2);
  const sharkMesh = new THREE.Mesh(
    sharkGeometry,
    new THREE.MeshStandardMaterial({
      color: 0xf87171,
      roughness: 0.48,
      metalness: 0.08,
    }),
  );
  scene.add(sharkMesh);

  const detectionSphere = new THREE.Mesh(
    new THREE.SphereGeometry(simulation.shark_detection_radius(), 32, 16),
    new THREE.MeshBasicMaterial({
      color: 0xf87171,
      wireframe: true,
      transparent: true,
      opacity: 0.08,
    }),
  );
  scene.add(detectionSphere);

  const targetLinePositions = new Float32Array(6);
  const targetLineGeometry = new THREE.BufferGeometry();
  targetLineGeometry.setAttribute(
    "position",
    new THREE.BufferAttribute(targetLinePositions, 3),
  );
  const targetLine = new THREE.Line(
    targetLineGeometry,
    new THREE.LineBasicMaterial({
      color: 0xf87171,
      transparent: true,
      opacity: 0.68,
    }),
  );
  targetLine.visible = false;
  scene.add(targetLine);

  const fishTransform = new THREE.Object3D();
  const forward = new THREE.Vector3(1, 0, 0);
  const scratchStart = new THREE.Vector3();
  const scratchEnd = new THREE.Vector3();

  function toSceneVector(x, y, z, target = new THREE.Vector3()) {
    return target.set(x - world.width * 0.5, world.height * 0.5 - y, z - world.depth * 0.5);
  }

  function updateCamera() {
    const horizontalDistance = Math.cos(cameraPitch) * cameraDistance;
    camera.position.set(
      Math.sin(cameraYaw) * horizontalDistance,
      Math.sin(cameraPitch) * cameraDistance,
      Math.cos(cameraYaw) * horizontalDistance,
    );
    camera.lookAt(cameraTarget);
  }

  function resize() {
    const { width, height } = canvas3d.getBoundingClientRect();
    const safeWidth = Math.max(1, width);
    const safeHeight = Math.max(1, height);
    renderer.setPixelRatio(window.devicePixelRatio || 1);
    renderer.setSize(safeWidth, safeHeight, false);
    camera.aspect = safeWidth / safeHeight;
    camera.updateProjectionMatrix();
    updateCamera();
  }

  function draw(state) {
    let fishCount = 0;
    for (let index = 0; index < state.fishPositions.length; index += 3) {
      toSceneVector(
        state.fishPositions[index],
        state.fishPositions[index + 1],
        state.fishPositions[index + 2],
        fishTransform.position,
      );
      fishTransform.updateMatrix();
      fishMesh.setMatrixAt(fishCount, fishTransform.matrix);
      fishCount += 1;
    }
    fishMesh.count = fishCount;
    fishMesh.instanceMatrix.needsUpdate = true;

    toSceneVector(...state.sharkPosition, sharkMesh.position);
    toSceneVector(...state.sharkPosition, detectionSphere.position);

    const heading = new THREE.Vector3(
      state.sharkHeading[0],
      -state.sharkHeading[1],
      state.sharkHeading[2],
    ).normalize();
    if (heading.lengthSq() > Number.EPSILON) {
      sharkMesh.quaternion.setFromUnitVectors(forward, heading);
    }

    if (state.sharkTarget.length === 3) {
      toSceneVector(...state.sharkPosition, scratchStart);
      toSceneVector(...state.sharkTarget, scratchEnd);

      targetLinePositions[0] = scratchStart.x;
      targetLinePositions[1] = scratchStart.y;
      targetLinePositions[2] = scratchStart.z;
      targetLinePositions[3] = scratchEnd.x;
      targetLinePositions[4] = scratchEnd.y;
      targetLinePositions[5] = scratchEnd.z;
      targetLineGeometry.attributes.position.needsUpdate = true;
      targetLine.visible = true;
    } else {
      targetLine.visible = false;
    }

    renderer.render(scene, camera);
  }

  let isDraggingCamera = false;
  let previousPointerX = 0;
  let previousPointerY = 0;

  canvas3d.addEventListener("pointerdown", (event) => {
    isDraggingCamera = true;
    previousPointerX = event.clientX;
    previousPointerY = event.clientY;
    canvas3d.setPointerCapture(event.pointerId);
  });

  canvas3d.addEventListener("pointermove", (event) => {
    if (!isDraggingCamera) {
      return;
    }

    const deltaX = event.clientX - previousPointerX;
    const deltaY = event.clientY - previousPointerY;
    previousPointerX = event.clientX;
    previousPointerY = event.clientY;
    cameraYaw -= deltaX * 0.006;
    cameraPitch = Math.max(-0.1, Math.min(0.95, cameraPitch - deltaY * 0.004));
    updateCamera();
  });

  canvas3d.addEventListener("pointerup", (event) => {
    isDraggingCamera = false;
    canvas3d.releasePointerCapture(event.pointerId);
  });

  canvas3d.addEventListener("wheel", (event) => {
    event.preventDefault();
    const zoomFactor = event.deltaY > 0 ? 1.08 : 0.92;
    cameraDistance = Math.max(
      world.height * 0.55,
      Math.min(Math.max(world.width, world.depth) * 1.7, cameraDistance * zoomFactor),
    );
    updateCamera();
  });

  updateCamera();

  return { draw, resize };
}

function readSimulationState() {
  return {
    dimensions: simulation.dimensions(),
    fishPositions: simulation.fish_positions(),
    sharkPosition: simulation.shark_position(),
    sharkHeading: simulation.shark_heading(),
    sharkTarget: simulation.shark_target_position(),
  };
}

function updateStats() {
  fishEatenValue.textContent = String(simulation.fish_eaten());
  fishEatenRateValue.textContent = simulation.fish_eaten_per_minute().toFixed(1);
  elapsedTimeValue.textContent = formatElapsedTime(simulation.elapsed_seconds());
}

function animate(currentTime) {
  const deltaSeconds = (currentTime - previousTime) / 1000;
  previousTime = currentTime;

  simulation.tick(deltaSeconds * simulationSpeed);
  updateStats();

  const state = readSimulationState();
  if (viewMode === "2d") {
    draw2d(state);
  } else {
    threeView.draw(state);
  }

  requestAnimationFrame(animate);
}

window.addEventListener("resize", resizeCanvases);
resizeCanvases();
requestAnimationFrame(animate);
