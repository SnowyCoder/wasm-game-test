import { WebApp }  from "wasm-test-02";


let app = WebApp.create();

let canvas = document.getElementById("canvas");

canvas.onclick = function() {
    if (document.pointerLockElement !== canvas) {
        canvas.requestPointerLock();
    } else {
        app.on_click();
    }
};

canvas.onkeyup = function(e) {
    app.on_key_up(e);
};

canvas.onkeydown = function(e) {
    app.on_key_down(e);
};


// Hook pointer lock state change events for different browsers
document.addEventListener('pointerlockchange', onLockChange, false);
window.addEventListener('resize', onResize, false);


function onLockChange() {
    if (document.pointerLockElement === canvas) {
        console.log('The pointer lock status is now locked');
        document.addEventListener("mousemove", onMouseMove, false);
    } else {
        console.log('The pointer lock status is now unlocked');
        document.removeEventListener("mousemove", onMouseMove, false);
    }
}

function onMouseMove(e) {
    app.on_mouse_move(e.movementX, e.movementY);
}

function onResize() {
    app.on_resize();
}

function update() {
    app.update(performance.now());
    requestAnimationFrame(update);
}

requestAnimationFrame(update);
