import { goto } from '$app/navigation';
import General from '$lib/layouts/admin-panel/general/General.svelte';
import Groups from '$lib/layouts/admin-panel/groups/Groups.svelte';
import Permissions from '$lib/layouts/admin-panel/permissions/Permissions.svelte';
import Sessions from '$lib/layouts/admin-panel/sessions/Sessions.svelte';
import Users from '$lib/layouts/admin-panel/users/Users.svelte';

import type { PageLoad } from './$types';

export type AdminPanelMenu =
    | "general"
    | "groups"
    | "permissions"
    | "users"
    | "sessions";

export const load: PageLoad = async ({ params }) => {
  let component

  switch (params.menu as AdminPanelMenu) {
      case "general":
          component = General;
          break
      case "groups":
          component = Groups;
          break
      case "permissions":
          component = Permissions;
          break
      case "users":
          component = Users;
          break
      case "sessions":
          component = Sessions;
          break
      default:
          throw goto("/admin-panel/general");
  }

  return { batata: "teste", component }
};
