<script lang="ts" setup>
import { ref } from "vue";

// const duration = ref<number>(0);
const message = ref<string>("");
const messageVisible = ref<boolean>(false);

const showMessage = (msg: string): void => {
  message.value = msg;
  messageVisible.value = true;
  setTimeout(() => {
    messageVisible.value = false;
  }, 2000);
};

const handleStart = (): void => {
  fetch("/api/start")
    .then((res) => res.json())
    .then((res) => {
      showMessage("已开启");
    });
};

const handleStop = (): void => {
  fetch("/api/stop")
    .then((res) => res.json())
    .then((res) => {
      showMessage("已关闭");
    });
};
</script>

<template>
  <div>
    <Transition name="fade" appear>
      <div class="message" v-if="messageVisible">{{ message }}</div>
    </Transition>
    <div class="buttonGroups">
      <div class="buttonStart" @click="handleStart">开启</div>
      <div class="buttonStop" @click="handleStop">关闭</div>
    </div>
  </div>
</template>

<style>
.message {
  position: absolute;
  top: 2rem;
  left: 50%;
  transform: translateX(-50%);
  padding: 10px 20px 10px 20px;
  background-color: white;
  border-radius: 3px;
  box-shadow: rgba(0, 0, 0, 0.12) 0px 3px 6px -4px,
    rgba(0, 0, 0, 0.08) 0px 6px 16px 0px, rgba(0, 0, 0, 0.05) 0px 9px 28px 8px;
  color: rgb(51, 54, 57);
  font-size: 14px;
  z-index: 100;
}

.buttonGroups {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 3rem;
}

.buttonStart {
  width: 10rem;
  height: 4rem;
  background-color: #cceada;
  color: #18a058;
  border-radius: 0.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}

.buttonStop {
  width: 10rem;
  height: 4rem;
  background-color: #f8d7da;
  color: #d03050;
  border-radius: 0.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
