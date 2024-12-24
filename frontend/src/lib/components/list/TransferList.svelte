<script lang="ts" context="module">
    export type TransferListItem = {
        id: string;
        label: string;
    };
</script>

<script lang="ts">
    import ListItem from "./ListItem.svelte";

    export let leftItems: TransferListItem[] = [];
    export let rightItems: TransferListItem[] = [];

    export let rightTitle = "Right";
    export let leftTitle = "Left";

    const onClickLeftItem = (item: { id: string; label: string }) => {
        rightItems = [...rightItems, item];
        leftItems = leftItems.filter((leftItem) => leftItem.id !== item.id);
    };

    const onClickRightItem = (item: { id: string; label: string }) => {
        leftItems = [...leftItems, item];
        rightItems = rightItems.filter((rightItem) => rightItem.id !== item.id);
    };
</script>

<div class="flex flex-col sm:flex-row gap-4 w-full">
    <div class="flex flex-col gap-2 w-full">
        <h4 class="text-sm font-medium text-gray-900">{leftTitle}</h4>

        {#each leftItems as item}
            <ListItem
                onClick={onClickLeftItem}
                id={item.id}
                label={item.label}
            />
        {/each}
    </div>

    <div class="flex flex-col gap-2 w-full">
        <h4 class="text-sm font-medium text-gray-900">{rightTitle}</h4>
        {#each rightItems as item}
            <ListItem
                onClick={onClickRightItem}
                id={item.id}
                label={item.label}
            />
        {/each}
    </div>
</div>
