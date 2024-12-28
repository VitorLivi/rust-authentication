<script module lang="ts">
    export type InputTextProps = {
        label?: string;
        placeholder?: string;
        required?: boolean;
        onChange?: (event: Event) => void;
        inputProps?: Record<string, unknown>;
    };
</script>

<script lang="ts">
    let props: InputTextProps = $props();
    const {
        label,
        onChange,
        placeholder,
        required = false,
        inputProps = {},
    } = props;
</script>

<div>
    {#if label}
        <label for={label} class="block text-sm font-medium text-gray-900">
            {label}
        </label>
    {/if}
    <input
        type="text"
        class="mt-1.5 w-full rounded-lg border-gray-300 text-gray-700 shadow-sm sm:text-sm p-3"
        placeholder={label ?? placeholder}
        {required}
        {...inputProps}
        oninput={onChange}
    />
</div>
