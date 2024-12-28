<script lang="ts" context="module">
    export type TransferListItem = {
        id: string;
        label: string;
    };
</script>

<script lang="ts">
    import { Input } from "flowbite-svelte";
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

    const onClickDoubleRightArrow = () => {
        rightItems = [...rightItems, ...leftItems];
        leftItems = [];
    };

    const onClickDoubleLeftArrow = () => {
        leftItems = [...leftItems, ...rightItems];
        rightItems = [];
    };

    const filterOptions = (target: HTMLInputElement) => {
        if (!target.parentElement?.parentElement) {
            return;
        }

        const items = target.parentElement.getElementsByTagName("button");

        if (!target.value) {
            for (let i = 0; i < items.length; i++) {
                items[i].style.display = "block";
            }
            return;
        }

        for (let i = 0; i < items.length; i++) {
            const item = items[i];
            const label = item.textContent;
            if (
                label &&
                !label.toLowerCase().includes(target.value.toLowerCase())
            ) {
                item.style.display = "none";
            } else {
                item.style.display = "block";
            }
        }
    };

    const onChangeLeftFilter = (event: Event) => {
        const target = event.target as HTMLInputElement;

        filterOptions(target);
    };

    const onChangeRightFilter = (event: Event) => {
        const target = event.target as HTMLInputElement;

        filterOptions(target);
    };
</script>

<div class={`flex flex-col sm:flex-row gap-4 w-full`}>
    <div class="flex flex-col gap-2 w-full">
        <h4 class="text-sm font-medium text-gray-900">{leftTitle}</h4>

        <Input type="text" oninput={onChangeLeftFilter} placeholder="filter" />
        {#each leftItems as item}
            <ListItem
                onClick={onClickLeftItem}
                id={item.id}
                label={item.label}
            />
        {/each}
    </div>

    <div class="flex flex-col justify-center mt-[70px]">
        <button
            disabled={leftItems.length === 0}
            on:click={onClickDoubleRightArrow}
            class="cursor-pointer block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700 text-left select-none disabled:opacity-50"
        >
            {">>"}
        </button>
        <button
            disabled={rightItems.length === 0}
            on:click={onClickDoubleLeftArrow}
            class="cursor-pointer block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700 text-left select-none disabled:opacity-50"
        >
            {"<<"}
        </button>
    </div>

    <div class="flex flex-col gap-2 w-full">
        <h4 class="text-sm font-medium text-gray-900">{rightTitle}</h4>

        <Input type="text" oninput={onChangeRightFilter} placeholder="filter" />
        {#each rightItems as item}
            <ListItem
                onClick={onClickRightItem}
                id={item.id}
                label={item.label}
            />
        {/each}
    </div>
</div>
