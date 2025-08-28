import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";


let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let fileList: HTMLElement | null;

import * as echarts from 'echarts';

// 基于准备好的dom，初始化echarts实例
var myChart = echarts.init(document.getElementById('form'));
// 绘制图表
myChart.setOption({
  title: {
    text: 'ECharts 入门示例'
  },
  tooltip: {},
  xAxis: {
    data: ['衬衫', '羊毛衫', '雪纺衫', '裤子', '高跟鞋', '袜子']
  },
  yAxis: {},
  series: [
    {
      name: '销量',
      type: 'bar',
      data: [5, 20, 36, 10, 10, 20]
    }
  ]
});


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

