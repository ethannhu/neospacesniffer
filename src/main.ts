import { open } from "@tauri-apps/plugin-dialog";
import { invoke, Channel } from '@tauri-apps/api/core';

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let fileList: HTMLElement | null;

import * as echarts from 'echarts';

type SniffEvent =
  | {
      event: 'found';
      data: {
        filePath: String,
        fileSize: number,
      };
    };

const onEvent = new Channel<SniffEvent>();
onEvent.onmessage = (message) => {
  console.log(`got test event: ${message.data.filePath}`);
};

await invoke('recursive_search', {
  onEvent,
});

