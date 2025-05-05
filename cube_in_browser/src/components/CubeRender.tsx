// src/components/MainPage.tsx
import React, { useEffect, useRef } from 'react';
import init, { run } from '../../pkg/cube_take_two';
import '../Card.css'
const CubeRenderComponent: React.FC = () => {

  const canvasRef = useRef<HTMLCanvasElement>(document.getElementById("cube-container") as HTMLCanvasElement);
  useEffect(() => {
    const initialize = async () => {
      console.log("begin");
      await init();
      console.log(window.devicePixelRatio, window.innerWidth, window.innerHeight);
      const canvas = canvasRef.current;
      run(
        window.devicePixelRatio,
        window.innerWidth,
        window.innerHeight,
        canvas
      );
    };
    initialize();
  }, []);



  return (
    <div className="card" id = 'cube-container'>
      <div className="card-artframe">
        <canvas
          ref={canvasRef}
          width={100}
          height={10}
          className="card-canvas"
        />
      </div>
      <div className="card-body">
        <div className="card-title">
          The Lost Cube
          </div>
        <div className="card-type">Artifact — WGPU Renderer</div>
        <div className="card-text">
          A lonely cube drifts through the void, seeking the fragments of its tesseract. Ironically, it's faces lacku the dimension to comprehend the truth it seeks. 
        </div>
      </div>
    </div>
  );


};

export default CubeRenderComponent;
