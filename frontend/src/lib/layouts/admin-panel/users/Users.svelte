<script lang="ts">
  import { Link, useLocation } from "svelte-routing";
  import { isRoute } from "$lib/utils/routing";
  import {
    Spinner,
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
  import { UserService } from "$lib/api/services/user/user";
  import { onMount } from "svelte";

  let loading = true;
  let users = [];

  const userService = new UserService();

  onMount(() => {
    userService
      .list()
      .then((res) => {
        users = res.data;
        console.log(users);
      })
      .catch((err) => {
        console.log(err);
      })
      .finally(() => {
        loading = false;
      });
  });

  let location = useLocation();
</script>

<div class="container mt-4">
  {#if isRoute("/admin-panel/users", $location)}
    {#if loading}
      <div class="w-full flex justify-center items-center h-[100px]">
        <div class="flex flex-row gap-4 items-center">
          <Spinner />
          <h1 class="text-primary-500 text-[24px] font-bold">
            Loading users...
          </h1>
        </div>
      </div>
    {/if}

    {#if !loading && users.length === 0}
      <div>No users found</div>
    {/if}

    {#if !loading && users.length > 0}
      <Table
        items={users}
        placeholder="Search by user"
        hoverable={true}
        filter={(item, searchTerm) =>
          item.username.toLowerCase().includes(searchTerm.toLowerCase())}
      >
        <TableHead>
          <TableHeadCell>ID</TableHeadCell>
          <TableHeadCell>First Name</TableHeadCell>
          <TableHeadCell>Last Name</TableHeadCell>
          <TableHeadCell>Email</TableHeadCell>
          <TableHeadCell>Username</TableHeadCell>
          <TableHeadCell>Action</TableHeadCell>
        </TableHead>
        <TableBody tableBodyClass="divide-y">
          <TableBodyRow slot="row" let:item>
            <TableBodyCell>{item.id}</TableBodyCell>
            <TableBodyCell>{item.first_name}</TableBodyCell>
            <TableBodyCell>{item.last_name}</TableBodyCell>
            <TableBodyCell>{item.email}</TableBodyCell>
            <TableBodyCell>{item.username}</TableBodyCell>
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
      <div class="flex flex-row justify-start mb-4 mt-4">
        <Link to="/admin-panel/users/add-user">
          <Button outline>Add User</Button>
        </Link>
      </div>
    {/if}
  {/if}

  {#if isRoute("/admin-panel/users/add-user", $location)}
    <AddUser />
  {/if}
</div>
