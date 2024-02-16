<script setup lang="ts">
import { ref } from 'vue'
import NewTodoView from '@/components/NewTodoView.vue'
import { Button } from '@/components/ui/button'
import TodosTable from '@/components/TodosTable.vue'
import type { ToDo } from '@/models/todo.ts'

const list = ref<ToDo[]>([])
const isOpen = ref(false)

async function fetchList() {
  const response = await fetch('http://localhost:6969/todos', { method: 'GET' })
  const json = await response.json()
  list.value = json
}
fetchList()

function onSaved() {
  isOpen.value = false
  fetchList()
}
</script>

<template>
  <main main class="container mt-4">
    <Button @click="isOpen = true">Create new ToDo</Button>
    <NewTodoView :open="isOpen" @update:open="(v) => (isOpen = v)" @saved="onSaved" />
    <TodosTable :list="list" @list:changed="fetchList" />
  </main>
</template>
