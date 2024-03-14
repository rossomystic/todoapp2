<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow
} from '@/components/ui/table'
import EditTodoView from '@/components/EditTodoView.vue'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Trash2Icon, PencilIcon } from 'lucide-vue-next'
import type { ToDo } from '@/models/todo'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

type Props = {
  list: ToDo[]
}

type Emits = {
  (e: 'list:changed'): void
}

const props = defineProps<Props>()
const emits = defineEmits<Emits>()

const editToDoOpen = ref<boolean>(false)
const editToDo = ref<ToDo>()

function onEditClick(item: ToDo) {
  editToDoOpen.value = true
  editToDo.value = item
}

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
  emits('list:changed')
}

function onEditSave() {
  editToDoOpen.value = false
  emits('list:changed')
}

async function deleteItem(id: number) {
  await fetch(`http://localhost:6969/todos/${id}`, {
    method: 'DELETE'
  })
  emits('list:changed')
}

const { push } = useRouter()
</script>

<template>
  <Table>
    <TableCaption> List of things to do </TableCaption>
    <TableHeader>
      <TableRow>
        <TableHead class="w-1/12"> </TableHead>
        <TableHead class="w-4/12"> Title </TableHead>
        <TableHead class="w-5/12"> Descrpition </TableHead>
        <TableHead class="w-2/12"> </TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow v-for="item in list" class="group" :key="`todo-${item.id}`">
        <TableCell>
          <Checkbox
            @click.stop="toggleComplete(item.id, !item.completed)"
            :checked="item.completed"
          />
        </TableCell>
        <TableCell class="cursor-pointer" @click="push(`/todos/${item.id}`)">
          {{ item.title }}
        </TableCell>
        <TableCell> {{ item.description }} </TableCell>
        <TableCell
          class="transition-opacity duration-500 opacity-0 group-hover:opacity-100 text-end"
        >
          <Button variant="ghost" size="icon" @click.stop="deleteItem(item.id)">
            <Trash2Icon class="text-red-500" />
          </Button>
          <Button variant="ghost" size="icon" @click.stop="onEditClick(item)">
            <PencilIcon />
          </Button>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
  <EditTodoView
    v-if="editToDo"
    :open="editToDoOpen"
    :todo="editToDo"
    @update:open="(v) => (editToDoOpen = v)"
    @saved="onEditSave"
  />
</template>
