<script lang="ts">
    export let text: string = "Button";
    export let size: "sm" | "md" | "lg" = "md";

    const getSize = () => {
        switch (size) {
            case "sm":
                return "px-1 py-1 text-xs";
            case "md":
                return "px-2 py-2 text-sm";
            case "lg":
                return "px-4 py-4 text-lg";
        }
    };
</script>

<button
    class={`inline-block rounded border border-indigo-600 text-indigo-600 hover:bg-indigo-600 hover:text-white focus:outline-none focus:ring active:bg-indigo-500 ${getSize()}`}
>
    {text}
</button>
