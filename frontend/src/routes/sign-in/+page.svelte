<script>
  import InputEmail from "$lib/components/input-email/InputEmail.svelte";
  import InputPassword from "$lib/components/input-password/InputPassword.svelte";
</script>

<div
  class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8 bg-white"
>
  <div class="sm:mx-auto sm:w-full sm:max-w-sm">
    <img
      class="mx-auto h-10 w-auto"
      src="https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=600"
      alt="Your Company"
    />
    <h2
      class="mt-10 text-center text-2xl/9 font-bold tracking-tight text-gray-900"
    >
      Sign in to your account
    </h2>
  </div>

  <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
    <form class="space-y-6" action="#" method="POST">
      <InputEmail label="email" required />

      <div>
        <InputPassword label="password" required />

        <div class="mt-4">
          <button
            type="submit"
            class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm/6 font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            >Sign in</button
          >
        </div>

        <div class="flex flex-col gap-2 mt-4">
          <button
            class="w-full px-4 py-2 border flex gap-2 border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-200 hover:border-slate-400 dark:hover:border-slate-500 hover:text-slate-900 dark:hover:text-slate-300 hover:shadow transition duration-150"
          >
            <img
              class="w-6 h-6"
              src="https://www.svgrepo.com/show/475656/google-color.svg"
              loading="lazy"
              alt="google logo"
            />
            <span>Login with Google</span>
          </button>

          <button
            class="w-full px-4 py-2 border flex gap-2 border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-200 hover:border-slate-400 dark:hover:border-slate-500 hover:text-slate-900 dark:hover:text-slate-300 hover:shadow transition duration-150"
          >
            <img
              class="w-6 h-6"
              src="https://www.svgrepo.com/show/475647/facebook-color.svg"
              loading="lazy"
              alt="facebook logo"
            />
            <span>Login with facebook</span>
          </button>
        </div>
      </div>
    </form>

    <p class="mt-10 text-center text-sm/6 text-gray-500">
      Not registered?
      <a
        href="/sign-up"
        class="font-semibold text-indigo-600 hover:text-indigo-500">Sign Up</a
      >
    </p>
  </div>
</div>
