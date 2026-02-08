
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;
const { platform } = window.__TAURI__.os;

console.log(platform())

function normalizePath(p) {
    if (!p) return p;
    return p.startsWith("file:///") ? p.slice(8) : p;
}

async function pickAudio() {
    let filePickerOptions = {
        multiple: false,
        directory: false,
        filters: [
          {
            name: "Audio Files",
            extensions: ["mp3", "wav", "ogg", "flac", "aac", "m4a"]
          }
        ]
    }
    if (platform() == "ios") {
        filePickerOptions = {
            multiple: false,
            directory: false
        }
    }
    const file = await open(filePickerOptions);
    document.getElementById("file-path").innerText = file
    document.getElementById("file-path-truncated").innerText = normalizePath(file)
    return normalizePath(file);
}
async function loadMetadata(path) {
    const metadata = await invoke("get_metadata", { path });
    console.log("Metadata:", metadata);
    return metadata;
}

window.addEventListener("DOMContentLoaded", () => {
    let pickElement = document.getElementById("pick-button");
    let metaElement = document.getElementById("meta-button");
    pickElement.addEventListener("click", async(e) => {
        e.preventDefault();
        const path = await pickAudio();  // await here!
        if (path) {
            await invoke("play_audio", { path });
        }
    });
    metaElement.addEventListener("click", async(e) => {
        e.preventDefault();
        const path = await pickAudio(); // await here too
        if (path) {
            const metadata = await loadMetadata(path);
            console.log(metadata);
        }
    });
});