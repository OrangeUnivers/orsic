
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

function normalizePath(p) {
    if (!p) return p;
    return p.startsWith("file:///") ? p.slice(8) : p;
}

async function pickAudio() {
    const file = await open({
        multiple: false,
        directory: false
        // recursive: false
    });
    document.getElementById("file-path").innerText = file
    document.getElementById("file-path-truncated").innerText = normalizePath(file)
    await invoke("play_audio", { path: normalizePath(file) });
    return file;
}

window.addEventListener("DOMContentLoaded", () => {
    let pickElement = document.getElementById("pick-button");
    pickElement.addEventListener("click", (e) => {
        e.preventDefault();
        pickAudio();
    });
});