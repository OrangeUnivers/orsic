
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

async function pickAudio() {
  const file = await open({
    multiple: false,
    filters: [
      {
        name: "Audio Files",
        extensions: ["mp3", "wav", "ogg", "flac", "aac", "m4a"]
      }
    ],
  });
  await invoke("play_audio", { path: file });
  return file;
}

window.addEventListener("DOMContentLoaded", () => {
  let pickElement = document.getElementById("pick-button");
  pickElement.addEventListener("click", (e) => {
    e.preventDefault();
    pickAudio();
  });
});