<script module lang="ts">
    export type InputTextProps = {
        label: string;
        placeholder?: string;
        required?: boolean;
        inputProps?: Record<string, unknown>;
    };
</script>

<script lang="ts">
    let props: InputTextProps = $props();
    const { label, placeholder, required = false, inputProps = {} } = props;
</script>

<label
    for={label}
    class="relative block rounded-md border border-gray-200 shadow-sm focus-within:border-blue-600 focus-within:ring-1 focus-within:ring-blue-600"
>
    <input
        type="text"
        class="p-2 peer border-none bg-transparent placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0"
        placeholder={label ?? placeholder}
        {required}
        {...inputProps}
    />

    <span
        class="pointer-events-none absolute start-2.5 top-0 -translate-y-1/2 bg-white p-0.5 text-xs text-gray-700 transition-all peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-sm peer-focus:top-0 peer-focus:text-xs"
    >
        {label}
    </span>
</label>
