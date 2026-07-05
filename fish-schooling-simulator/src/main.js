import init, { Simulation } from "../pkg/fish_schooling_simulator.js";
import "./styles.css";

const canvas = document.querySelector("#simulation");
const context = canvas.getContext("2d");

await init();

const simulation = new Simulation();
let previousTime = performance.now();

function resizeCanvas() {
  const pixelRatio = window.devicePixelRatio || 1;
  canvas.width = Math.floor(window.innerWidth * pixelRatio);
  canvas.height = Math.floor(window.innerHeight * pixelRatio);
  canvas.style.width = `${window.innerWidth}px`;
  canvas.style.height = `${window.innerHeight}px`;
  context.setTransform(pixelRatio, 0, 0, pixelRatio, 0, 0);
}

function worldTransform() {
  const worldWidth = simulation.world_width();
  const worldHeight = simulation.world_height();
  const scale = Math.min(
    window.innerWidth / worldWidth,
    window.innerHeight / worldHeight,
  );
  const offsetX = (window.innerWidth - worldWidth * scale) * 0.5;
  const offsetY = (window.innerHeight - worldHeight * scale) * 0.5;

  return { scale, offsetX, offsetY, worldWidth, worldHeight };
}

function screenPoint(x, y, transform) {
  return {
    x: transform.offsetX + x * transform.scale,
    y: transform.offsetY + y * transform.scale,
  };
}

function drawFish(x, y, transform) {
  const point = screenPoint(x, y, transform);
  const radius = Math.max(2.2, 3.8 * transform.scale);
  context.beginPath();
  context.arc(point.x, point.y, radius, 0, Math.PI * 2);
  context.fill();
}

function drawShark(x, y, headingX, headingY, transform) {
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

function draw() {
  const transform = worldTransform();

  context.clearRect(0, 0, window.innerWidth, window.innerHeight);
  context.fillStyle = "#071923";
  context.fillRect(0, 0, window.innerWidth, window.innerHeight);

  context.strokeStyle = "rgba(155, 216, 217, 0.22)";
  context.lineWidth = 1;
  context.strokeRect(
    transform.offsetX,
    transform.offsetY,
    transform.worldWidth * transform.scale,
    transform.worldHeight * transform.scale,
  );

  const [sharkX, sharkY] = simulation.shark_position();
  const [headingX, headingY] = simulation.shark_heading();
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

  const sharkTarget = simulation.shark_target_position();
  if (sharkTarget.length === 2) {
    const targetPoint = screenPoint(sharkTarget[0], sharkTarget[1], transform);
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
  const positions = simulation.fish_positions();
  for (let index = 0; index < positions.length; index += 2) {
    drawFish(positions[index], positions[index + 1], transform);
  }

  context.fillStyle = "#f87171";
  drawShark(sharkX, sharkY, headingX, headingY, transform);
}

function animate(currentTime) {
  const deltaSeconds = (currentTime - previousTime) / 1000;
  previousTime = currentTime;

  simulation.tick(deltaSeconds);
  draw();
  requestAnimationFrame(animate);
}

window.addEventListener("resize", resizeCanvas);
resizeCanvas();
requestAnimationFrame(animate);
