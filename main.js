import init from "./webcasting_wasm/pkg/webcasting_wasm.js";

//startup
var canvas = document.getElementById("canvas");
canvas.height = canvas.style.height = window.innerHeight;
canvas.width = canvas.style.width = window.innerWidth;
var aspectRatio = canvas.width / canvas.height;
var ctx = canvas.getContext("2d");
ctx.imageSmoothingEnabled = false;
var imageData;


setTimeout(await (init()), 1000);
//canvas.addEventListener("click", handleInput);
