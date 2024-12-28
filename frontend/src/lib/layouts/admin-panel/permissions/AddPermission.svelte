<script lang="ts">
    import { Button, Input } from "flowbite-svelte";

    const handleSubmit = (e: SubmitEvent) => {
        e.preventDefault();
        console.log("submit");
    };
</script>

<form onsubmit={handleSubmit} class="space-y-4 flex flex-col gap-4">
    <div class="grid grid-cols-1 gap-4">
        <div>
            <Input type="text" placeholder="Name" />
        </div>
        <div class="flex w-full justify-start">
            <Button class="w-[150px]" type="submit" outline
                >Save</Button
            >
        </div>
    </div>
</form>
