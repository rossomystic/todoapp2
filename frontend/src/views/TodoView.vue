<script setup lang="ts">
import type { ToDo } from '@/models/todo'
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import { CheckCircleIcon, Trash2Icon } from 'lucide-vue-next'
import { CircleIcon } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { useRouter } from 'vue-router'

type Emits = {
  (e: 'list:changed'): void
}

const emits = defineEmits<Emits>()

const { params } = useRoute()
const id = params.id as string

const todo = ref<ToDo>()
async function fetchItem() {
  const response = await fetch(`http://localhost:6969/todos/${id}`, { method: 'GET' })
  const json = await response.json()
  todo.value = json
}

fetchItem()

async function toggleComplete(id: number, value: boolean) {
  await fetch(`http://localhost:6969/todos/${id}`, {
    method: 'PATCH',
    body: JSON.stringify({
      completed: value
    }),
    headers: {
      'Content-Type': 'application/json'
    }
  })
  fetchItem()
}
const { replace } = useRouter()

async function deleteItem(id: number) {
  await fetch(`http://localhost:6969/todos/${id}`, {
    method: 'DELETE'
  })
  emits('list:changed')
  replace(`/todos`)
}
</script>

<template>
  <main v-if="todo">
    <section class="relative h-[26rem]">
      <img
        class="absolute inset-0 object-cover w-full h-full"
        :src="`https://picsum.photos/seed/{todo.id}/1920/416`"
      />
      <div
        class="absolute inset-x-0 bottom-0 z-10 py-4 text-white top-1/2 bg-gradient-to-t from-black/80 to-transparent"
      >
        <div class="container flex items-end justify-between h-full">
          <h1 class="text-5xl font-bold">{{ todo.title }}</h1>
          <Button
            v-if="todo.completed"
            variant="ghost"
            @click="toggleComplete(todo.id, false)"
            class="flex items-center gap-2 text-lg text-green-500"
          >
            <CheckCircleIcon stroke-width="3" class="w-8 h-6" />Completed
          </Button>
          <Button
            v-else
            variant="ghost"
            @click="toggleComplete(todo.id, true)"
            class="flex items-center gap-2 text-lg text-neutral-400"
          >
            <CircleIcon stroke-width="3" class="w-8 h-6" />Not Completed
          </Button>
          <Button
            variant="ghost"
            class="flex items-center gap-2 text-lg text-red-400"
            @click.stop="deleteItem(todo.id)"
          >
            <Trash2Icon stroke-width="3" class="w-8 h-6" />Delete
          </Button>
        </div>
      </div>
    </section>
    <section class="container py-2">
      <p class="text-lg font-light">{{ todo.description }}</p>
    </section>
  </main>
</template>
