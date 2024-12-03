<script lang="ts">
    import Button from "$lib/components/button/Button.svelte";
    import Table from "$lib/components/table/Table.svelte";
    import { page } from "$app/stores";
    import { Link } from "svelte-routing";
</script>

<div class="bg-white p-2 lg:col-span-3">
    <div class="flex flex-row justify-start mb-4 mt-8">
        <Link to="/admin-panel/groups/add-group">
            <Button text="Add Group" />
        </Link>
    </div>

    <!-- <Route path="/admin-panel/groups/add-group" component={AddGroup} /> -->

    {#if $page.url.pathname === "/admin-panel/groups"}
        <Table />
    {/if}

    <!-- <form action="#" class="space-y-4">
        <div>
            <label class="sr-only" for="name">Name</label>
            <input
                class="w-full rounded-lg border-gray-200 p-3 text-sm"
                placeholder="Name"
                type="text"
                id="name"
            />
        </div>

        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div>
                <label class="sr-only" for="email">Email</label>
                <input
                    class="w-full rounded-lg border-gray-200 p-3 text-sm"
                    placeholder="Email address"
                    type="email"
                    id="email"
                />
            </div>

            <div>
                <label class="sr-only" for="phone">Phone</label>
                <input
                    class="w-full rounded-lg border-gray-200 p-3 text-sm"
                    placeholder="Phone Number"
                    type="tel"
                    id="phone"
                />
            </div>
        </div>
    </form> -->
</div>
