<script context="module">
    export type SelectOption = {
        value: string;
        label: string;
    };
</script>

<script lang="ts">
    export let label: string;
    export let id: string;

    export let options: SelectOption[] = [];
</script>

<div>
    <label for={id} class="block text-sm font-medium text-gray-900">
        {label}
    </label>

    <select
        name={id}
        {id}
        class="mt-1.5 w-full rounded-lg border-gray-300 shadow-sm text-gray-700 sm:text-sm p-3 bg-white"
    >
        {#each options as option}
            <option value={option.value}>{option.label}</option>
        {/each}
    </select>
</div>
