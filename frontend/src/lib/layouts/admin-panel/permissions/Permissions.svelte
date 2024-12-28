<script lang="ts">
  import { Link, useLocation } from "svelte-routing";
  import AddPermission from "./AddPermission.svelte";
  import { isRoute } from "$lib/utils/routing";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { Button } from "flowbite-svelte";
  let items = [
    { id: 1, maker: "Toyota", type: "ABC", make: 2017 },
    { id: 2, maker: "Ford", type: "CDE", make: 2018 },
    { id: 3, maker: "Volvo", type: "FGH", make: 2019 },
    { id: 4, maker: "Saab", type: "IJK", make: 2020 },
  ];

  let location = useLocation();

  $: {
    console.log($location);
  }
</script>

{#if isRoute("/admin-panel/permissions", $location)}
  <div class="flex flex-row justify-start mb-4 mt-4">
    <Link to="/admin-panel/permissions/add-permission">
      <Button outline>Add Permission</Button>
    </Link>
  </div>

  <Table
    {items}
    placeholder="Search by permission"
    hoverable={true}
    filter={(item, searchTerm) =>
      item.maker.toLowerCase().includes(searchTerm.toLowerCase())}
  >
    <TableHead>
      <TableHeadCell>ID</TableHeadCell>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>Created Date</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      <TableBodyRow slot="row" let:item>
        <TableBodyCell>{item.id}</TableBodyCell>
        <TableBodyCell>{item.maker}</TableBodyCell>
        <TableBodyCell>{item.type}</TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
{/if}

{#if isRoute("/admin-panel/permissions/add-permission", $location)}
  <AddPermission />
{/if}
