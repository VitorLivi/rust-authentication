<script lang="ts" module>
    import { Link } from "svelte-routing";
</script>

<div class="flex h-full flex-col justify-between shadow-md bg-white">
    <div class="px-4 py-6">
        <ul class="mt-6 space-y-1">
            <li>
                <Link to="/admin-panel/general">
                    <div
                        class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                    >
                        General
                    </div>
                </Link>
            </li>

            <li>
                <Link to="/admin-panel/groups">
                    <div
                        class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                    >
                        Groups
                    </div>
                </Link>
            </li>

            <li>
                <Link to="/admin-panel/permissions">
                    <div
                        class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                    >
                        Permissions
                    </div>
                </Link>
            </li>

            <li>
                <Link to="/admin-panel/users">
                    <div
                        class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                    >
                        Users
                    </div>
                </Link>
            </li>

            <li>
                <Link to="/admin-panel/sessions">
                    <div
                        class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                    >
                        Sessions
                    </div>
                </Link>
            </li>
        </ul>
    </div>
</div>
