<script lang="ts">
    import { Button, Datepicker, Input } from "flowbite-svelte";
</script>

<form action="#" class="space-y-4 flex flex-col gap-4">
    <div class="grid gap-4 grid-cols-3">
        <Input type="text" placeholder="Username" />
        <Input type="text" placeholder="Full Name" />
        <Input type="email" placeholder="Email" />
    </div>
    <div class="grid gap-4 grid-cols-[2fr_2fr_1fr]">
        <Input type="password" placeholder="Password" />
        <Input type="password" placeholder="Confirm Password" />
        <Datepicker placeholder="Birth Date" />
    </div>
    <div class="flex items-center mb-4">
        <input
            id="default-checkbox"
            type="checkbox"
            value=""
            class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
        />
        <label
            for="default-checkbox"
            class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
            >Ask for new password on first login</label
        >
    </div>

    <div class="flex w-full justify-start">
        <Button class="w-[150px]" type="submit" outline
            >Save</Button
        >
    </div>
</form>
