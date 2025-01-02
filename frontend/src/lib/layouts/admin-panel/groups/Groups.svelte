<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { Link, useLocation } from "svelte-routing";
  import AddGroup from "./AddGroup.svelte";
  import { writable } from "svelte/store";
  import {
    SvelteFlow,
    Controls,
    Background,
    BackgroundVariant,
    MiniMap,
  } from "@xyflow/svelte";

  import "@xyflow/svelte/dist/style.css";
  import { isRoute } from "$lib/utils/routing";

  const nodes = writable([
    {
      id: "1",
      type: "default",
      data: { label: "Input Node" },
      position: { x: 0, y: 0 },
    },
    {
      id: "2",
      type: "default",
      data: { label: "Node" },
      position: { x: 0, y: 150 },
    },
  ]);

  const edges = writable([
    {
      id: "1-2",
      type: "default",
      source: "1",
      target: "2",
      label: "Edge Text",
    },
  ]);

  const snapGrid = [25, 25];

  let location = useLocation();
</script>

<div class="bg-white p-2 lg:col-span-3 container mt-4">
  {#if isRoute("/admin-panel/groups", $location)}
    <div style:height="500px">
      <SvelteFlow
        {nodes}
        {edges}
        {snapGrid}
        nodesConnectable={false}
        fitView
        on:nodeclick={(event) =>
          console.log("on node click", event.detail.node)}
      >
        <Controls />
        <Background variant={BackgroundVariant.Dots} />
        <MiniMap />
      </SvelteFlow>
    </div>
  {/if}

  {#if $location.pathname !== "/admin-panel/groups/add-group"}
    <div class="flex flex-row justify-start mb-4 mt-4">
      <Link to="/admin-panel/groups/add-group">
        <Button outline>Add Group</Button>
      </Link>
    </div>
  {/if}

  {#if isRoute("/admin-panel/groups/add-group", $location)}
    <AddGroup />
  {/if}
</div>
