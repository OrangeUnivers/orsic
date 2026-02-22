const { platform } = require("@tauri-apps/plugin-os");

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;
const { BaseDirectory, exists, mkdir, writeTextFile, readTextFile } = window.__TAURI__.fs;

const currentPlatform = ""
const baseFileOperations = currentPlatform == "ios" ? BaseDirectory.Document : BaseDirectory.AppData

async function secureMainFolder() {
    const appFolder = "com.orangeunivers.orsic";
    if (currentPlatform == "ios") { return; }

    if (!(await exists(appFolder, { baseDir: BaseDirectory.Data }))) {
        await mkdir(appFolder, { baseDir: BaseDirectory.Data, recursive: true })
    }
}

async function loadDataFile() {
    const file = "data.json";
    await secureMainFolder();

    const fileExists = await exists(file, { baseDir: baseFileOperations });
    if (!fileExists) {
        const defaultData = {
            "$schema": "https://raw.githubusercontent.com/OrangeUnivers/orsic/0.2.0-alpha.2/schemas/data.json",
            "tracks": {}
        }
        await writeTextFile(file, JSON.stringify(defaultData), { baseDir: baseFileOperations });
        return defaultData;
    }

    const text = await readTextFile(file, { baseDir: baseFileOperations });
    return JSON.parse(text);
}

async function saveDataFile(data) {
    const file = "data.json";
    await writeTextFile(file, JSON.stringify(data), { baseDir: baseFileOperations });
}

let dataFile = await loadDataFile();
console.log("Data: ", dataFile);

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
    if (currentPlatform == "ios") {
        filePickerOptions = {
            multiple: false,
            directory: false
        }
    }
    const file = await open(filePickerOptions);
    return normalizePath(file);
}

async function loadMetadata(path) {
    const metadata = await invoke("get_metadata", { path });
    console.log("Metadata:", metadata);
    return metadata;
}

function addToGallery(musicPath) {

}

let onImport = "ask";
window.addEventListener("DOMContentLoaded", () => {
    const openNewTrackButton = document.getElementById("open-track");
    const importDialogHolder = document.getElementById("import-dialog-holder");
    const importDialogCheckbox = document.getElementById("import-dialog-checkbox");
    const importDialogCheckboxButton = document.getElementById("import-dialog-checkbox-button");
    const importDialogGallery = document.getElementById("import-dialog-gallery");
    const importDialogOnce = document.getElementById("import-dialog-once");
    openNewTrackButton.addEventListener("click", async(e) => {
        e.preventDefault();
        const path = await pickAudio();  // await here!
        if (path) {
            switch (onImport) {
                case "ask":
                    importDialogHolder.classList.add("open");
                    break;
                case "gallery":
                    // do gallery stuff
                    break;
                case "once":
                    // do playin' stuff
                    break;
            }
        }
    });
    importDialogCheckbox.addEventListener("click", () => {
        if (importDialogCheckboxButton.hasAttribute("checked")) {
            importDialogCheckboxButton.removeAttribute("checked", "");
        } else {
            importDialogCheckboxButton.setAttribute("checked", "");
        }
    });
    importDialogGallery.addEventListener("click", () => {
        if (importDialogCheckboxButton.hasAttribute("checked")) {
            onImport = "gallery";
        }
        importDialogHolder.classList.remove("open");
        // do gallery stuff
    });
    importDialogOnce.addEventListener("click", () => {
        if (importDialogCheckboxButton.hasAttribute("checked")) {
            onImport = "once";
        }
        importDialogHolder.classList.remove("open");
        // do playin' stuf
    });
    // metaElement.addEventListener("click", async(e) => {
    //     e.preventDefault();
    //     const path = await pickAudio(); // await here too
    //     if (path) {
    //         const metadata = await loadMetadata(path);
    //         console.log(metadata);
    //     }
    // });
});

// let hue = 0
// setInterval(() => {
//     document.documentElement.style.setProperty("--c-accent-h", hue);
//     hue += 1;
//     if (hue == 360) {
//         hue = 0;
//     }
// }, 10);