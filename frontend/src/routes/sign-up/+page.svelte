<script lang="ts">
    import { AuthService } from "$lib/api/services/auth/auth";

    type SignUpFormData = {
        username: string;
        email: string;
        password: string;
        first_name: string;
        last_name: string;
    };

    function getFormData(form: HTMLFormElement): SignUpFormData | null {
        const formData = new FormData(form);
        let data: { [key: string]: string } = {};

        const requiredFields = [
            "username",
            "email",
            "password",
            "first_name",
            "last_name",
        ];

        for (const [key, value] of formData.entries()) {
            console.log(key, value);

            data[key as keyof SignUpFormData] = value as string;
        }

        for (const field of requiredFields) {
            if (!data[field]) {
                return null;
            }
        }

        return data as SignUpFormData;
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
        authService.signUp(data).then(() => {
            window.location.href = "/sign-in";
        });
    }
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
            Sign Up
        </h2>
    </div>

    <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
        <form class="space-y-6" on:submit={submitForm}>
            <div>
                <label
                    for="username"
                    class="block text-sm/6 font-medium text-gray-900"
                    >Username</label
                >
                <div class="mt-2">
                    <input
                        id="username"
                        name="username"
                        type="text"
                        required
                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6"
                    />
                </div>
            </div>
            <div>
                <label
                    for="email"
                    class="block text-sm/6 font-medium text-gray-900"
                    >Email</label
                >
                <div class="mt-2">
                    <input
                        id="email"
                        name="email"
                        type="email"
                        autocomplete="email"
                        required
                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6"
                    />
                </div>
            </div>
            <div>
                <div class="flex items-center justify-between">
                    <label
                        for="password"
                        class="block text-sm/6 font-medium text-gray-900"
                        >Password</label
                    >
                </div>
                <div class="mt-2">
                    <input
                        id="password"
                        name="password"
                        type="password"
                        autocomplete="current-password"
                        required
                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6"
                    />
                </div>
            </div>
            <div>
                <div class="flex items-center justify-between">
                    <label
                        for="first_name"
                        class="block text-sm/6 font-medium text-gray-900"
                        >First Name</label
                    >
                </div>
                <div class="mt-2">
                    <input
                        id="first_name"
                        name="first_name"
                        type="text"
                        required
                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6"
                    />
                </div>
            </div>
            <div>
                <div class="flex items-center justify-between">
                    <label
                        for="last_name"
                        class="block text-sm/6 font-medium text-gray-900"
                        >Last Name</label
                    >
                </div>
                <div class="mt-2">
                    <input
                        id="last_name"
                        name="last_name"
                        type="text"
                        required
                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6"
                    />
                </div>
            </div>

            <div>
                <button
                    type="submit"
                    class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm/6 font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >Confirm</button
                >
            </div>
        </form>
    </div>
</div>
