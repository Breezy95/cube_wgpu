//import * as init from '../../pkg/cube_take_two_bg.wasm'
export abstract class App{
    abstract close(): boolean;
    constructor(){
        console.log('HIT')
    }
}