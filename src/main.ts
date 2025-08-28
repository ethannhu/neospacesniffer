import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";


let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let fileList: HTMLElement | null;


async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function refreshLoop() {
  while (true) {
    if (fileList) {
      const buffer:ArrayBuffer = await invoke("fetch_files");
      const decoder = new TextDecoder();
      const jsonStr = decoder.decode(new Uint8Array(buffer));
      const restored = JSON.parse(jsonStr);

      console.log(restored);
    }
    // TODO: Render Chart
    await sleep(2000);
  }
}

async function selectRoot() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  if (selected) {
    console.log("Selected: ", selected);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
    selectRoot();
  });
  fileList = document.querySelector("#file-list");
  refreshLoop()
});

