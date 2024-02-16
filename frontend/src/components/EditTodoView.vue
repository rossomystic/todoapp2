<script setup lang="ts">
import { TODO_SCHEMA } from './TodoForm.vue'
import { toTypedSchema } from '@vee-validate/zod'
import TodoForm from './TodoForm.vue'
import { Form } from '@/components/ui/form'
import { Button } from '@/components/ui/button'

import {
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  Dialog,
  DialogContent
} from '@/components/ui/dialog'

import type { ToDo } from '@/models/todo'

type Props = {
  open: boolean
  todo: ToDo
}

type Emits = {
  (e: 'update:open', value: boolean): void
  (e: 'saved'): void
}

const props = defineProps<Props>()
const emits = defineEmits<Emits>()

const schema = toTypedSchema(TODO_SCHEMA)

async function onSubmit(values: any) {
  const id = props.todo.id
  await fetch(`http://localhost:6969/todos/${id}`, {
    method: 'PATCH',
    body: JSON.stringify(values),
    headers: {
      'Content-type': 'application/json'
    }
  })
  emits('saved')
}
</script>
<template>
  <Dialog :open="open" @update:open="(v) => emits('update:open', v)">
    <DialogContent>
      <DialogHeader>
        <DialogTitle>New To Do</DialogTitle>
        <DialogDescription> Create a new To Do item </DialogDescription>
      </DialogHeader>
      <Form
        class="flex flex-col items-start gap-8 my-3"
        @submit="onSubmit"
        :initialValues="todo"
        :validation-schema="schema"
      >
        <TodoForm></TodoForm>
        <DialogFooter>
          <Button @click="emits('update:open', false)" variant="ghost" type="button">Cancel</Button>
          <Button type="submit">Save</Button>
        </DialogFooter>
      </Form>
    </DialogContent>
  </Dialog>
</template>
