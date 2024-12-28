<script lang="ts">
    import TransferList from "$lib/components/list/TransferList.svelte";
    import {
        Button,
        Input,
        Select,
        type SelectOptionType,
    } from "flowbite-svelte";

    const groupOptions: SelectOptionType<any>[] = [
        { value: "1", name: "Group 1" },
        { value: "2", name: "Group 2" },
        { value: "3", name: "Group 3" },
    ];

    const leftItems = [
        { id: "1", label: "Member 1" },
        { id: "2", label: "Member 2" },
        { id: "3", label: "Member 3" },
    ];

    const rightItems = [{ id: "4", label: "Member 4" }];

    const handleSubmit = (e: SubmitEvent) => {
        e.preventDefault();
        console.log("submit");
    };
</script>

<form onsubmit={handleSubmit} class="space-y-4 flex flex-col gap-4">
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
            <Input type="text" placeholder="Name" />
        </div>

        <Select id="parent-groups" placeholder="Parent" items={groupOptions} />
    </div>

    <TransferList
        leftTitle="Available Members"
        rightTitle="Members"
        {leftItems}
        {rightItems}
    />

    <div class="flex w-full justify-start">
        <Button class="w-[150px]" type="submit" outline
            >Save</Button
        >
    </div>
</form>
