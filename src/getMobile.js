const { platform } = window.__TAURI__.os;

currentPlatform = platform();
if (currentPlatform == "ios" || currentPlatform == "android") {
    document.documentElement.dataset.mobile = "";
}