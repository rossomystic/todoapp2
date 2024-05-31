<script lang="ts">
import z, { type TypeOf } from 'zod'
import { toTypedSchema } from '@vee-validate/zod'
import { Form } from '@/components/ui/form'
import { Button } from '@/components/ui/button'

const LOGIN_SCHEMA = z.object({
  username: z.string().min(3).max(100),
  password: z.string().max(100)
})

type LoginSchema = z.infer<typeof LOGIN_SCHEMA>
</script>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { Input } from '@/components/ui/input'
import { FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form'

const schema = toTypedSchema(LOGIN_SCHEMA)
const router = useRouter()

async function onSubmit(values: LoginSchema) {
  const response = await fetch('http://localhost:6969/auth/signin', {
    method: 'POST',
    body: JSON.stringify(values),
    headers: {
      'Content-type': 'application/json'
    }
  })
  const token = await response.text()
  localStorage.setItem('TOKEN', token)
  router.replace({ name: 'todos' })
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
