// src/components/MainPage.tsx
import React, { useEffect } from 'react';
import init, { run } from '../../pkg/cube_take_two';

const CubeRenderComponent: React.FC = () => {
  useEffect(() => {
    const initialize = async () => {
      console.log("begin");
      await init();
      console.log(window.devicePixelRatio, window.innerWidth, window.innerHeight);
      run(
        window.devicePixelRatio,
        window.innerWidth,
        window.innerHeight,
        document.getElementById("cube-container") as HTMLCanvasElement
      );
    };
    initialize();
  }, []);

  const handleSourceClick = () => {
    window.location.href = "https://github.com/Breezy95/cube_wgpu";
  };

  return (
    
    <div>
      <div className="button-controls">
        <button
          className="welcome-button"
          id="source"
          onClick={handleSourceClick}
        >
          Source
        </button>
      </div>
      <div className='card'>
        <div className='card-canvas-div'>
      <canvas className="" id="cube-container" />
        </div>
        <div className='card-content'>
          klkjlkjkljklj

        </div>
      </div>
    </div>
  );
};

/*
<div class="card">
    <div class="card-canvas" id="render-area">
      <!-- Insert your <canvas> or WebGPU render target here -->
      <canvas id="renderCanvas" width="320" height="200"></canvas>
    </div>
    <div class="card-content">
      <div class="card-title">My Render</div>
      <div class="card-text">This is a simple card layout with a render area on top.</div>
    </div>
  </div>
*/
export default CubeRenderComponent;
