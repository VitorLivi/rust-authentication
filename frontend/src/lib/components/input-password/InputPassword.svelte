<script lang="ts">
    import InputText, {
        type InputTextProps,
    } from "../input-text/InputText.svelte";

    const props: InputTextProps = $props();

    const complementedProps = {
        ...props,
        inputProps: { ...props.inputProps, type: "password" },
    };
</script>

<InputText {...complementedProps} />
