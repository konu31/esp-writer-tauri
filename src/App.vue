<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";


const portList = ref([
  { text: 'Select', value: '', disabled: true },
  { text: 'Port1', value: '1', disabled: false },
  { text: 'Port2', value: '2', disabled: false },
  { text: 'Port3', value: '3', disabled: false }
])

const baudrateList = ref([
  { text: '921600', value: '921600', disabled: false },
  { text: '460800', value: '460800', disabled: false },
  { text: '230400', value: '230400', disabled: false },
  { text: '115200', value: '115200', disabled: false }
])

const selectedPort = ref("")
const selectedBaudrate = ref("921600")
const writeable = ref(false)
const result = ref("")

async function getBoardInfo() {
  console.log("getBoardInfo")
  result.value = await invoke("get_board_info", {
    port: selectedPort.value
  });

  writeable.value = true
}

async function writeFirmware() {
  console.log("write")
  result.value = await invoke("write_firmware", {
    port: selectedPort.value,
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
          <select class="form-select" id="selectPort" v-model="selectedPort" @change="getBoardInfo">
            <option v-for="option in portList" :value="option.value" :disabled="option.disabled">
              {{ option.text }}
            </option>
          </select>
        </div>
      </div>
      <div class="row mb-3">
        <label for="selectBaudrate" class="col-sm-2 col-form-label">Baudrate</label>
        <div class="col-sm-10">
          <select class="form-select" id="selectBaudrate" v-model="selectedBaudrate">
            <option v-for="option in baudrateList" :value="option.value" :disabled="option.disabled">
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
      <textarea class="form-control" id="result" rows="15" readonly v-model="result"></textarea>
    </div>
  </main>
</template>

<style scoped>

</style>
<style>

</style>