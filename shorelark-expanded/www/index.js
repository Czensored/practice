import * as sim from "../libs/simulation-wasm/pkg/lib_simulation_wasm.js";
const SPRITE_SIZE = 0.012;

function cssColorFromPackedRGBA(packed) {
  const u = packed >>> 0; // ensure unsigned
  const r = (u >>> 24) & 255;
  const g = (u >>> 16) & 255;
  const b = (u >>> 8)  & 255;
  const a =  u         & 255;
  return `rgba(${r},${g},${b},${a / 255})`;
}

CanvasRenderingContext2D.prototype.drawTriangle =
  function(x, y, size, rotation) {
    this.beginPath();

    this.moveTo(
      x - Math.sin(rotation) * size * 1.5,
      y + Math.cos(rotation) * size * 1.5,
    );

    this.lineTo(
      x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
      y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
      x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
      y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
      x - Math.sin(rotation) * size * 1.5,
      y + Math.cos(rotation) * size * 1.5,
    );

    // this.stroke();
    // this.fillStyle = 'rgb(255, 255, 255)';
    this.fill();
  };

CanvasRenderingContext2D.prototype.drawCircle =
  function(x, y, radius) {
    this.beginPath();

    this.arc(x, y, radius, 0, 2.0 * Math.PI);

    this.fillStyle = 'rgb(0, 255, 128)';
    this.fill();
  };

const simulation = new sim.Simulation();

document.getElementById('train').onclick = function() {
  const count = parseInt(document.getElementById('fastForwardCount').value) || 1;
  for (let i = 0; i < count; i++) {
    console.log(simulation.fast_forward());
  }
};

const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;
viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const ctxt = viewport.getContext('2d');

ctxt.scale(viewportScale, viewportScale);

ctxt.fillStyle = 'rgb(0, 0, 0)';

function redraw() {
  ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

  const stats = simulation.step();
  if (stats) {
    console.log(stats);
  }

  const world = simulation.world();

  for (const food of world.foods) {
    ctxt.drawCircle(
      food.x * viewportWidth,
      food.y * viewportHeight,
      (SPRITE_SIZE / 2.0) * viewportWidth,
    );
  }

  for (const animal of world.animals) {
    ctxt.fillStyle = cssColorFromPackedRGBA(animal.color);
    ctxt.drawTriangle(
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      SPRITE_SIZE * viewportWidth,
      animal.rotation,
    );
  }

  requestAnimationFrame(redraw);
}

redraw();
