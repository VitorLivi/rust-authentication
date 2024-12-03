<script lang="ts">
    export let breadcrumbs: {
        name: string;
        path: string;
    }[];
</script>

<nav
    class="flex items-center space-x-2 text-sm text-gray-500 p-4 bg-gray-100 w-full"
>
    {#each breadcrumbs as breadcrumb, index}
        <a
            href={breadcrumb.path}
            class="hover:text-blue-600 transition-colors"
            class:font-bold={index === breadcrumbs.length - 1}
        >
            {breadcrumb.name}
        </a>

        {#if index !== breadcrumbs.length - 1}
            <span>/</span>
        {/if}
    {/each}
</nav>
