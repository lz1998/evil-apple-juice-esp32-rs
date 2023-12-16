<script setup lang="ts">
import { ref } from "vue";

const handleStart = () => {
  fetch("/api/start");
};

const handleStop = () => {
  fetch("/api/stop");
};
</script>

<template>
  <div class="buttonGroups">
    <div class="buttonStart" @click="handleStart">Start</div>
    <div class="buttonStop" @click="handleStop">Stop</div>
  </div>
</template>

<style>
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
</style>
