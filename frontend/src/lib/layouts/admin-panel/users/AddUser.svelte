<script lang="ts">
  import { Button, Datepicker, Helper, Input } from "flowbite-svelte";
  import { UserService } from "$lib/api/services/user/user";
  import toast from "svelte-french-toast";

  let selectedDate = "";
  let inputStates: {
    [key: string]: {
      error: boolean;
      helpertext: null | string;
      placeholder: string;
    };
  } = {
    username: {
      error: false,
      helpertext: null,
      placeholder: "Username",
    },
    email: {
      error: false,
      helpertext: null,
      placeholder: "Email",
    },
    password: {
      error: false,
      helpertext: null,
      placeholder: "Password",
    },
    confirmPassword: {
      error: false,
      helpertext: null,
      placeholder: "Confirm Password",
    },
    firstName: {
      error: false,
      helpertext: null,
      placeholder: "First Name",
    },
    lastName: {
      error: false,
      helpertext: null,
      placeholder: "Last Name",
    },
    birthDate: {
      error: false,
      helpertext: null,
      placeholder: "Birth Date",
    },
    askForNewPassword: {
      error: false,
      helpertext: null,
      placeholder: "Ask for new password on first login",
    },
  };

  function resetInputState(inputName: string): void {
    inputStates[inputName].error = false;
    inputStates[inputName].helpertext = null;
  }

  function resetAllInputStates(): void {
    for (const key in inputStates) {
      resetInputState(key);
    }
  }

  const onSelectBirthDate = (event: CustomEvent<any>) => {
    selectedDate = event.detail;
  };

  type AddUserFormData = {
    username: string;
    email: string;
    password: string;
    confirmPassword: string;
    firstName: string;
    lastName: string;
    birthDate: string;
    askForNewPassword: string;
  };

  function resetForm(form: HTMLFormElement): void {
    form.reset();
    selectedDate = "";
  }

  function getFormData(form: HTMLFormElement): AddUserFormData | null {
    const formData = new FormData(form);
    let data: { [key: string]: string } = {};

    const requiredFields = [
      "askForNewPassword",
      "confirmPassword",
      "email",
      "firstName",
      "lastName",
      "password",
      "username",
      "birthDate",
    ];

    formData.set("birthDate", selectedDate);

    let someFieldIsEmpty = false;
    for (const [key, value] of formData.entries()) {
      data[key as keyof AddUserFormData] = value as string;

      if (requiredFields.includes(key) && !value) {
        inputStates[key].error = true;
        inputStates[key].helpertext = "This field is required";
        someFieldIsEmpty = true;
      }
    }

    if (someFieldIsEmpty) {
      return null;
    }

    if (data.password !== data.confirmPassword) {
      inputStates.confirmPassword.error = true;
      inputStates.confirmPassword.helpertext = "Passwords do not match";
      return null;
    }

    return data as AddUserFormData;
  }

  function submitForm(event: SubmitEvent): void {
    event.preventDefault();
    event.stopPropagation();
    resetAllInputStates();

    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }

    const form: HTMLFormElement = event.target;
    const data = getFormData(form);

    if (!data) {
      return;
    }

    const askForNewPassword = data.askForNewPassword === "on" ? true : false;

    const userService = new UserService();
    const createUserPromisse = userService
      .create({
        username: data.username,
        email: data.email,
        password: data.password,
        first_name: data.firstName,
        last_name: data.lastName,
        birth_date: data.birthDate,
        ask_for_new_password: askForNewPassword,
      })
      .then(() => {
        resetForm(form);
      })
      .catch((error) => {
        console.error(error);
      });

    toast.promise(
      createUserPromisse,
      {
        loading: "Creating user...",
        success: "User created successfully",
        error: "An error occurred while creating the user",
      },
      { position: "top-right" },
    );
  }

  const errorDatepickerClass =
    "bg-red-50 border border-red-700  text-red-900 placeholder-red-700 text-sm rounded-lg focus:ring-red-500 focus:border-red-500 block w-full dark:bg-red-700 dark:border-red-600 dark:placeholder-red-400 dark:text-white dark:focus:ring-red-500 dark:focus:border-red-500";
</script>

<form
  on:submit={submitForm}
  class="space-y-4 flex flex-col gap-4 container mt-4"
>
  <div class="grid gap-4 grid-cols-4">
    <div>
      <Input
        id="username"
        name="username"
        type="text"
        color={inputStates.username.error ? "red" : undefined}
        placeholder={inputStates.username.placeholder}
      />
      {#if inputStates.username.error}
        <Helper class="mt-2" color="red">
          <span class="font-medium">{inputStates.username.helpertext}</span>
        </Helper>
      {/if}
    </div>
    <div>
      <Input
        id="firstName"
        name="firstName"
        type="text"
        color={inputStates.firstName.error ? "red" : undefined}
        placeholder={inputStates.firstName.placeholder}
      />
      {#if inputStates.firstName.error}
        <Helper class="mt-2" color="red">
          <span class="font-medium">{inputStates.firstName.helpertext}</span>
        </Helper>
      {/if}
    </div>
    <div>
      <Input
        id="lastName"
        name="lastName"
        type="text"
        color={inputStates.lastName.error ? "red" : undefined}
        placeholder="Last Name"
      />
      {#if inputStates.lastName.error}
        <Helper class="mt-2" color="red">
          <span class="font-medium">{inputStates.lastName.helpertext}</span>
        </Helper>
      {/if}
    </div>
    <div>
      <Input
        id="email"
        name="email"
        type="email"
        color={inputStates.email.error ? "red" : undefined}
        placeholder="Email"
      />
      {#if inputStates.email.error}
        <Helper class="mt-2" color="red">
          <span class="font-medium">{inputStates.email.helpertext}</span>
        </Helper>
      {/if}
    </div>
  </div>
  <div class="grid gap-4 grid-cols-[2fr_2fr_1fr]">
    <div>
      <Input
        id="password"
        name="password"
        type="password"
        color={inputStates.password.error ? "red" : undefined}
        placeholder="Password"
      />
      {#if inputStates.password.error}
        <Helper class="mt-2" color="red">
          <span class="font-medium">{inputStates.password.helpertext}</span>
        </Helper>
      {/if}
    </div>
    <div>
      <Input
        id="confirmPassword"
        name="confirmPassword"
        type="password"
        color={inputStates.confirmPassword.error ? "red" : undefined}
        placeholder="Confirm Password"
      />
      {#if inputStates.confirmPassword.error}
        <Helper class="mt-2" color="red">
          <span class="font-medium"
            >{inputStates.confirmPassword.helpertext}</span
          >
        </Helper>
      {/if}
    </div>
    <div>
      <Datepicker
        color={inputStates.birthDate.error ? "red" : undefined}
        inputClass={inputStates.birthDate.error
          ? errorDatepickerClass
          : undefined}
        on:select={onSelectBirthDate}
        placeholder="Birth Date"
      />

      {#if inputStates.birthDate.error}
        <Helper class="mt-2" color="red">
          <span class="font-medium">{inputStates.birthDate.helpertext}</span>
        </Helper>
      {/if}
    </div>
  </div>
  <div class="flex items-center mb-4">
    <input
      id="askForNewPassword"
      name="askForNewPassword"
      type="checkbox"
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
