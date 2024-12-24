<script lang="ts" module>
    import ListItem from "$lib/components/list/ListItem.svelte";
    import { Link } from "svelte-routing";

    const menuItems = [
        {
            label: "General",
            link: "/admin-panel/general",
        },
        {
            label: "Groups",
            link: "/admin-panel/groups",
        },
        {
            label: "Permissions",
            link: "/admin-panel/permissions",
        },
        {
            label: "Users",
            link: "/admin-panel/users",
        },
        {
            label: "Sessions",
            link: "/admin-panel/sessions",
        },
    ];
</script>

<div class="flex h-full flex-col justify-between shadow-md bg-white">
    <div class="px-4 py-6">
        <ul class="mt-6 space-y-1">
            {#each menuItems as menuItem}
                <li>
                    <Link to={menuItem.link}>
                        <ListItem label={menuItem.label} />
                    </Link>
                </li>
            {/each}
        </ul>
    </div>
</div>
