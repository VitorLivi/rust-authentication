<script lang="ts">
    import InputText from "$lib/components/input-text/InputText.svelte";
    import TransferList from "$lib/components/list/TransferList.svelte";
    import type { SelectOption } from "$lib/components/select/Select.svelte";
    import Select from "$lib/components/select/Select.svelte";

    const groupOptions: SelectOption[] = [
        { value: "1", label: "Group 1" },
        { value: "2", label: "Group 2" },
        { value: "3", label: "Group 3" },
    ];

    const leftItems = [
        { id: "1", label: "Member 1" },
        { id: "2", label: "Member 2" },
        { id: "3", label: "Member 3" },
    ];

    const rightItems = [{ id: "4", label: "Member 4" }];
</script>

<form action="#" class="space-y-4 flex flex-col gap-4">
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
            <InputText label="Name" />
        </div>

        <Select id="parent-groups" label="Parent" options={groupOptions} />
    </div>

    <TransferList
        leftTitle="Available Members"
        rightTitle="Members"
        {leftItems}
        {rightItems}
    />
</form>
