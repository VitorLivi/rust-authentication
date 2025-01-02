<script lang="ts">
  import { Button, Datepicker, Input } from "flowbite-svelte";
  import { AuthService } from "$lib/api/services/auth/auth";

  let selectedDate = "";

  const onSelectBirthDate = (event: CustomEvent<any>) => {
  console.log(event)
    selectedDate = event.detail.date;
  };

  type AddUserFormData = {
    username: string;
    email: string;
    password: string;
    fistName: string;
    lastName: string;
    birthDate: string;
    askForNewPassword: string;
  };

  function getFormData(form: HTMLFormElement): AddUserFormData | null {
    const formData = new FormData(form);
    let data: { [key: string]: string } = {};

    const requiredFields = [
      "username",
      "email",
      "password",
      "fistName",
      "lastName",
      "askForNewPassword",
    ];

    for (const [key, value] of formData.entries()) {
      data[key as keyof AddUserFormData] = value as string;
    }

    for (const field of requiredFields) {
      if (!data[field]) {
        return null;
      }
    }

    if (!selectedDate) {
      return null;
    }

    data.birthDate = selectedDate;
    return data as AddUserFormData;
  }

  function submitForm(event: SubmitEvent): void {
    event.preventDefault();
    event.stopPropagation();

    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }

    const form: HTMLFormElement = event.target;
    const data = getFormData(form);

    if (!data) {
      return;
    }

    const authService = new AuthService();
    authService.create(data).then(() => {
      alert("User created successfully");
    });
  }
</script>

<form
  onsubmit={submitForm}
  action="#"
  class="space-y-4 flex flex-col gap-4 container mt-4"
>
  <div class="grid gap-4 grid-cols-4">
    <Input id="username" type="text" placeholder="Username" />
    <Input id="firstName" type="text" placeholder="First Name" />
    <Input id="lastName" type="text" placeholder="Last Name" />
    <Input id="email" type="email" placeholder="Email" />
  </div>
  <div class="grid gap-4 grid-cols-[2fr_2fr_1fr]">
    <Input id="password" type="password" placeholder="Password" />
    <Input
      id="confirmPassword"
      type="password"
      placeholder="Confirm Password"
    />
    <Datepicker on:select={onSelectBirthDate} placeholder="Birth Date" />
  </div>
  <div class="flex items-center mb-4">
    <input
      id="askForNewPassword"
      type="checkbox"
      value="false"
      class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
    />
    <label
      for="askForNewPassword"
      class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
      >Ask for new password on first login</label
    >
  </div>

  <div class="flex w-full justify-start">
    <Button class="w-[150px]" type="submit" outline>Save</Button>
  </div>
</form>
