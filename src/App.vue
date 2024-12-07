<script setup lang="ts">
import { onBeforeMount, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";


const portList = ref([
  { text: "Select", value: "", disabled: true }
])

const chipList = ref([
  { text: "", value: "" },  
  { text: "ESP32", value: "esp32" },
  { text: "ESP32-C3", value: "esp32c3" },
  { text: "ESP32-S3", value: "esp32s3" }
])

const baudrateList = ref([
  { text: "921600", value: "921600" },
  { text: "460800", value: "460800" },
  { text: "230400", value: "230400" },
  { text: "115200", value: "115200" }
])

let gettingPort = ref(true)
let selectedPort = ref("")
let selectedChip = ref("")
let selectedBaudrate = ref("921600")
let writeable = ref(false)
let result = ref("")


/**
 * ポート取得中に更新ボタンをクリックできないようにする
 */
watch(portList, (newValue, _oldValue) => {
  gettingPort.value = newValue.length <= 1
});


/**
 * コンソール(textarea)の最終行が表示されるようにスクロール
 */
function scrollResult() {
  let textarea = document.getElementById("result")
  if (textarea) {
    textarea.scrollTop = textarea.scrollHeight
  }
}

/**
 * get_port_listの結果からPortのプルダウンを作成
 */
listen("port_list", (event) => {
  const message: string = String(event.payload)
  if (message.startsWith("error:")) {
    result.value += message
    scrollResult()
  } else {
    const portListJson = JSON.parse(message)
    portList.value = [{ text: "Select", value: "", disabled: true }]
    portListJson.forEach((device: string) => {
      portList.value.push({ text: device, value: device, disabled: false })
    })
  }
})

/**
 * get_board_infoの結果をコンソール(textarea)に出力 & Chipを選択
 */
listen("board_info", (event) => {
  const message: string = String(event.payload)
  result.value += message
  scrollResult()

  const detectChipMessage: string = "Detecting chip type... "
  if (message.startsWith(detectChipMessage)) {
    const chip = message.replace(detectChipMessage, "").slice(0, -1)
    chipList.value.forEach(item => {
      if (item.text === chip) {
        selectedChip.value = item.value
        writeable.value = true
      }
    })
  }
})

/**
 * write_firmwareの結果を受け取りコンソール(textarea)に出力
 */
listen("write_firmware", (event) => {
  result.value += String(event.payload)
  scrollResult()
})


/**
 * ポートリスト取得
 */
async function getPortList() {
  portList.value = [
    { text: "Select", value: "", disabled: true }
  ]
  await invoke("get_port_list")
}

/**
 * 画面生成時イベント
 */
onBeforeMount(async () => {
  await getPortList()
})

/**
 * ボード情報取得
 */
async function getBoardInfo() {
  result.value = ""
  await invoke("get_board_info", {
    port: selectedPort.value
  });
}

/**
 * ファームウェア書き込み
 */
async function writeFirmware() {
  result.value = ""
  await invoke("write_firmware", {
    port: selectedPort.value,
    chip: selectedChip.value,
    baudrate: selectedBaudrate.value
  });
}
</script>

<template>
  <main class="container">
    <form>
      <div class="row my-3">
        <label for="selectPort" class="col-sm-2 col-form-label">Port</label>
        <div class="col-sm-10">
          <div class="input-group">
            <select class="form-select" id="selectPort" v-model="selectedPort" @change="getBoardInfo">
              <option v-for="option in portList" :value="option.value" :disabled="option.disabled">
                {{ option.text }}
              </option>
            </select>
            <button class="btn btn-outline-secondary" type="button" :disabled="gettingPort" @click="getPortList">
              <span v-if="gettingPort" class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
              <i v-else class="bi bi-arrow-clockwise"></i>
            </button>
          </div>
        </div>
      </div>

      <div class="row mb-3">
        <label for="selectChip" class="col-sm-2 col-form-label">Chip</label>
        <div class="col-sm-10">
          <select class="form-select" id="selectChip" v-model="selectedChip" disabled>
            <option v-for="option in chipList" disabled :value="option.value">
              {{ option.text }}
            </option>
          </select>
        </div>
      </div>

      <div class="row mb-3">
        <label for="selectBaudrate" class="col-sm-2 col-form-label">Baudrate</label>
        <div class="col-sm-10">
          <select class="form-select" id="selectBaudrate" v-model="selectedBaudrate">
            <option v-for="option in baudrateList" :value="option.value">
              {{ option.text }}
            </option>
          </select>
        </div>
      </div>
    </form>

    <div class="mb-3 d-grid gap-2">
      <button v-if="writeable" class="btn btn-primary" type="button" @click="writeFirmware">Write</button>
      <button v-else class="btn btn-secondary" type="button" disabled>Write</button>
    </div>

    <div class="mb-3">
      <textarea class="form-control" id="result" rows="12" readonly v-model="result"></textarea>
    </div>
  </main>
</template>

<style scoped>

</style>
<style>

</style>