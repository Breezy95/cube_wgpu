import wasm from "vite-plugin-wasm";
import {defineConfig} from 'vite'
//import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
    wasm(),
    //topLevelAwait()
  ],
  resolve: {
    alias: {
        pkg: "/src/pkg",
    },
  },
  build: {
    target: "ES2022"
  },
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      allow: ['..'],
    },
  },
});