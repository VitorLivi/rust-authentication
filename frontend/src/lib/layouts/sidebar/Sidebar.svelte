<script lang="ts" module>
    import { Link } from "svelte-routing";
</script>

<div class="flex h-full flex-col justify-between border-e bg-white">
    <div class="px-4 py-6">
        <ul class="mt-6 space-y-1">
            <li>
                <div
                    class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                >
                    <Link to="/admin-panel/general">General</Link>
                </div>
            </li>

            <li>
                <div
                    class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                >
                    <Link to="/admin-panel/groups">Groups</Link>
                </div>
            </li>

            <li>
                <div
                    class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                >
                    <Link to="/admin-panel/permissions">Permissions</Link>
                </div>
            </li>

            <li>
                <div
                    class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                >
                    <Link to="/admin-panel/users">Users</Link>
                </div>
            </li>

            <li>
                <div
                    class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                >
                    <Link to="/admin-panel/sessions">Sessions</Link>
                </div>
            </li>
        </ul>
    </div>
</div>
