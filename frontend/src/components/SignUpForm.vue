<script lang="ts">
import z, { type TypeOf } from 'zod'
import { toTypedSchema } from '@vee-validate/zod'
import { Form } from '@/components/ui/form'
import { Button } from '@/components/ui/button'

const SIGNUP_SCHEMA = z.object({
  name: z.string().min(3),
  surname: z.string().min(3),
  email: z.string().min(1).email('Insert a valid email'),
  username: z.string().min(3).max(100),
  password: z.string().max(100)
})

type SignUpSchema = z.infer<typeof SIGNUP_SCHEMA>
</script>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { Input } from '@/components/ui/input'
import { FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form'

const schema = toTypedSchema(SIGNUP_SCHEMA)
const router = useRouter()

async function onSubmit(values: SignUpSchema) {
  const response = await fetch('http://localhost:6969/auth/signup', {
    method: 'POST',
    body: JSON.stringify(values),
    headers: {
      'Content-type': 'application/json'
    }
  })
  router.replace({ name: 'auth' })
}
</script>

<template>
  <Form class="flex flex-col items-start gap-8 my-3" @submit="onSubmit" :validation-schema="schema">
    <FormField name="name" v-slot="{ componentField }">
      <FormItem class="w-full">
        <FormLabel>Name</FormLabel>
        <FormControl>
          <Input class="w-full" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>
    <FormField name="surname" v-slot="{ componentField }">
      <FormItem class="w-full">
        <FormLabel>Surname</FormLabel>
        <FormControl>
          <Input class="w-full" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>
    <FormField name="email" v-slot="{ componentField }">
      <FormItem class="w-full">
        <FormLabel>Email</FormLabel>
        <FormControl>
          <Input type="email" class="w-full" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>
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
          <Input type="password" class="w-full" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>

    <Button type="submit">Login</Button>
  </Form>
</template>
