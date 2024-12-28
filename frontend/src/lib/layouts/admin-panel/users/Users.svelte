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
  import { Button } from "flowbite-svelte";
  import AddUser from "./AddUser.svelte";
  import TableEditIcon from "$lib/components/table/TableEditIcon.svelte";
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

{#if isRoute("/admin-panel/users", $location)}
  <div class="flex flex-row justify-start mb-4 mt-4">
    <Link to="/admin-panel/users/add-user">
      <Button outline>Add User</Button>
    </Link>
  </div>

  <Table
    {items}
    placeholder="Search by user"
    hoverable={true}
    filter={(item, searchTerm) =>
      item.maker.toLowerCase().includes(searchTerm.toLowerCase())}
  >
    <TableHead>
      <TableHeadCell>ID</TableHeadCell>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>Email</TableHeadCell>
      <TableHeadCell>Created Date</TableHeadCell>
      <TableHeadCell>Last Login</TableHeadCell>
      <TableHeadCell>Action</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      <TableBodyRow slot="row" let:item>
        <TableBodyCell>{item.id}</TableBodyCell>
        <TableBodyCell>{item.maker}</TableBodyCell>
        <TableBodyCell>{item.maker}</TableBodyCell>
        <TableBodyCell>{item.type}</TableBodyCell>
        <TableBodyCell>{item.type}</TableBodyCell>
        <TableBodyCell>
          <div class="flex flex-row gap-2">
            <Link to="/admin-panel/users/edit-user">
              <TableEditIcon />
            </Link>

            <TableDeleteIcon />
          </div>
        </TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
{/if}

{#if isRoute("/admin-panel/users/add-user", $location)}
  <AddUser />
{/if}
