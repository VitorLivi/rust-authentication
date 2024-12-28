<script lang="ts">
    export let id: string;
    export let label: string;
    export let onClick:
        | ((item: { id: string; label: string }) => void)
        | undefined = undefined;
</script>

<button
    class="cursor-pointer block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700 text-left select-none"
    on:click={() => onClick?.({ id, label })}
>
    {label}
</button>
