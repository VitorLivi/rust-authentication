<script lang="ts">
  import { Link, Route, useLocation } from "svelte-routing";
  import { isRoute } from "$lib/utils/routing";
  import {
    Modal,
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
  import EditUser from "./EditUser.svelte";
  import { ExclamationCircleOutline } from "flowbite-svelte-icons";
  import toast from "svelte-french-toast";
  import type { ListUserOutput } from "$lib/api/services/user/types";

  let loading = $state(true);
  let isModalOpen = $state({
    value: false,
    id: "",
  });

  let users: (ListUserOutput & { isDeleting: boolean })[] = $state([]);

  onMount(() => {
    const userService = new UserService();
    userService
      .list()
      .then((res) => {
        users = res.data.map((user: any) => {
          user.isDeleting = false;

          return user;
        });
      })
      .catch((err) => {
        console.log(err);
      })
      .finally(() => {
        loading = false;
      });
  });

  let location = useLocation();

  function onClickDeleteButton(id: string): void {
    isModalOpen = {
      value: true,
      id,
    };
  }

  function onCancelDelete(): void {
    isModalOpen = {
      value: false,
      id: "",
    };
  }

  function onConfirmDelete(): void {
    const userService = new UserService();
    let usersCopy = [...users];

    const userIndex = usersCopy.findIndex((user) => user.id === isModalOpen.id);

    if (userIndex === -1) {
      return;
    }

    usersCopy[userIndex].isDeleting = true;

    const deletePromisse = userService
      .delete(isModalOpen.id)
      .then(() => {
        usersCopy = usersCopy.filter((user) => user.id !== isModalOpen.id);
        users = [...usersCopy];
      })
      .catch((err) => {
        console.error(err);
        usersCopy[userIndex].isDeleting = false;
        users = [...usersCopy];

        return Promise.reject(err);
      })
      .finally(() => {
        isModalOpen = {
          value: false,
          id: "",
        };
      });

    toast.promise(
      deletePromisse,
      {
        loading: "Deleting user...",
        success: "User deleted successfully",
        error: "An error occurred while deleting the user",
      },
      { position: "top-right" },
    );
  }
</script>

<Modal bind:open={isModalOpen.value} size="xs" autoclose>
  <div class="text-center">
    <ExclamationCircleOutline
      class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
    />
    <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
      Are you sure you want to delete this user?
    </h3>
    <Button onclick={onConfirmDelete} color="red" class="me-2"
      >Yes, I'm sure</Button
    >
    <Button onclick={onCancelDelete} color="alternative">No, cancel</Button>
  </div>
</Modal>

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

    {#if !loading}
      {#if users.length === 0}
        <h1 class="text-primary-500">No users found</h1>
      {:else}
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
            <TableHeadCell>Birth Date</TableHeadCell>
            <TableHeadCell>Action</TableHeadCell>
          </TableHead>
          <TableBody tableBodyClass="divide-y">
            <TableBodyRow slot="row" let:item>
              <TableBodyCell>{item.id}</TableBodyCell>
              <TableBodyCell>{item.first_name}</TableBodyCell>
              <TableBodyCell>{item.last_name}</TableBodyCell>
              <TableBodyCell>{item.email}</TableBodyCell>
              <TableBodyCell>{item.username}</TableBodyCell>
              <TableBodyCell>{item.birth_date ?? "-"}</TableBodyCell>
              <TableBodyCell>
                <div class="flex flex-row gap-2">
                  <Link to={`/admin-panel/users/edit-user/${item.id}`}>
                    <TableEditIcon />
                  </Link>

                  <button
                    disabled={item.isDeleting}
                    onclick={() => onClickDeleteButton(item.id)}
                  >
                    <TableDeleteIcon />
                  </button>
                </div>
              </TableBodyCell>
            </TableBodyRow>
          </TableBody>
        </Table>
      {/if}
      <div class="flex flex-row justify-start mb-4 mt-4">
        <Link to="/admin-panel/users/add-user">
          <Button outline>Add User</Button>
        </Link>
      </div>
    {/if}
  {/if}

  <Route path="/admin-panel/users/add-user" component={AddUser} />
  <Route path="/admin-panel/users/edit-user/:id" component={EditUser} />
</div>
