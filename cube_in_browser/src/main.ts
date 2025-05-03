//import './style.css'
//import typescriptLogo from './typescript.svg'
//import viteLogo from '/vite.svg'
//import {App} from './app.ts'


import init, {run}  from '../pkg/cube_take_two'
//import init, {run} from 'cube_render_wasm'


export class MainPage {
  close(): boolean {
    return true;
  }
    constructor(){
      //super()
      document.getElementById
      this.buttonControls()
      this.canvasCreator()
      //run(window.devicePixelRatio, window.innerWidth, window.innerHeight, null)
      console.log(window.devicePixelRatio, window.innerWidth, window.innerHeight)
       requestAnimationFrame(() => {
            run(window.devicePixelRatio, window.innerWidth, window.innerHeight,  document.getElementById("cube-container") as HTMLCanvasElement);
    });
     // run(window.devicePixelRatio, window.innerWidth, window.innerHeight, document.getElementById("rustyCanvas") as HTMLCanvasElement)
   }

   private buttonControls(): void {
    let container = document.createElement("div")
    container.className = "button-controls"
    container.appendChild(this.createButton("Source", () => {
      location.href = "https://github.com/Breezy95/cube_wgpu"
    }))
    document.body.appendChild(container)

   }
   
   private canvasCreator(): void {
    let cont = document.createElement("canvas")
    cont.id = "cube-container"
    cont.tabIndex = 3
    document.body.appendChild(cont)
   }
   
   private createButton(label: string, clickFn: () => void): HTMLButtonElement{
        let button = document.createElement("button")
        button.onclick = clickFn
        button.innerText = label
        button.id = label.split(" ").join("-").toLowerCase()
        button.className = "welcome-button"
        console.log("button")
        return button
  }
}



window.onload = async () => {
  console.log("begin")
  await init();
  new MainPage()
}
