<script lang="ts">
import z from 'zod'
import { toTypedSchema } from '@vee-validate/zod'
import { Form } from '@/components/ui/form'
import { Button } from '@/components/ui/button'

const LOGIN_SCHEMA = z.object({
  username: z.string().min(3).max(100),
  password: z.string().min(8).max(100)
})
</script>

<script setup lang="ts">
import { Input } from '@/components/ui/input'
import { FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form'

const schema = toTypedSchema(LOGIN_SCHEMA)

type Emits = {
  (e: 'update:open', value: boolean): void
  (e: 'saved'): void
}

const emits = defineEmits<Emits>()

async function onSubmit(values: any) {
  const body = {
    ...values,
    completed: false
  }

  await fetch('http://localhost:6969/auth', {
    method: 'POST',
    body: JSON.stringify(body),
    headers: {
      'Content-type': 'application/json'
    }
  })
  emits('saved')
}
</script>

<template>
  <Form class="flex flex-col items-start gap-8 my-3" @submit="onSubmit" :validation-schema="schema">
    <FormField name="username" v-slot="{ componentField }">
      <FormItem class="w-full">
        <FormLabel>Username</FormLabel>
        <FormControl>
          <Input class="w-full" placeholder="" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>
    <FormField name="password" v-slot="{ componentField }">
      <FormItem class="w-full">
        <FormLabel>Password</FormLabel>
        <FormControl>
          <Input class="w-full" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>
    <Button type="submit">Login</Button>
  </Form>
</template>
