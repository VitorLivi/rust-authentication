<script lang="ts">
  import { Link, useLocation } from "svelte-routing";
  import { isRoute } from "$lib/utils/routing";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import TableDeleteIcon from "$lib/components/table/TableDeleteIcon.svelte";
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

{#if isRoute("/admin-panel/sessions", $location)}
  <Table
    {items}
    placeholder="Search by sessions"
    hoverable={true}
    filter={(item, searchTerm) =>
      item.maker.toLowerCase().includes(searchTerm.toLowerCase())}
  >
    <TableHead>
      <TableHeadCell>ID</TableHeadCell>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>Logged At</TableHeadCell>
      <TableHeadCell>Expires At</TableHeadCell>
      <TableHeadCell>Action</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      <TableBodyRow slot="row" let:item>
        <TableBodyCell>{item.id}</TableBodyCell>
        <TableBodyCell>{item.maker}</TableBodyCell>
        <TableBodyCell>{item.type}</TableBodyCell>
        <TableBodyCell>{item.type}</TableBodyCell>
        <TableBodyCell>
          <TableDeleteIcon />
        </TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
{/if}
