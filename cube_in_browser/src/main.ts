import './style.css'
//import typescriptLogo from './typescript.svg'
//import viteLogo from '/vite.svg'
//import {App} from './app.ts'

//import init from '../../pkg/cube_take_two_bg.wasm'
import init, {run} from 'cube_render_wasm'


export class MainPage {
  close(): boolean {
    return true;
  }
    constructor(){
      //super()
      console.log(init)
      this.buttonControls()
      run(window.devicePixelRatio,window.innerWidth, window.innerHeight)
   }

   private buttonControls(): void {
    let container = document.createElement("div")
    container.className = "button-controls"
    container.appendChild(this.createButton("Source", () => {
      location.href = "https://github.com/Breezy95/cube_wgpu"
    }))
    document.body.appendChild(container)

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
  await init();
  new MainPage()
}
