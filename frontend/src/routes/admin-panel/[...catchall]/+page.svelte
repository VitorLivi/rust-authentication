<script lang="ts">
  import Groups from "$lib/layouts/admin-panel/groups/Groups.svelte";
  import Permissions from "$lib/layouts/admin-panel/permissions/Permissions.svelte";
  import Sessions from "$lib/layouts/admin-panel/sessions/Sessions.svelte";
  import Users from "$lib/layouts/admin-panel/users/Users.svelte";
  import Navbar from "$lib/layouts/navbar/Navbar.svelte";
  import Sidebar from "$lib/layouts/sidebar/Sidebar.svelte";
  import { Route, Router } from "svelte-routing";
  import "./style.css";
  import Breadcrumb from "$lib/components/breadcrumb/Breadcrumb.svelte";
  import { adminPanelBreadcrumbs } from "$lib/data/breadcrumb/admin-panel";
  import General from "$lib/layouts/admin-panel/general/General.svelte";
</script>

<div class="flex flex-col h-full">
  <Navbar />
  <div id="admin-panel-global">
    <Router>
      <Sidebar />

      <div id="admin-panel-content-wrapper">
        <Breadcrumb breadcrumbs={adminPanelBreadcrumbs} />

        <div id="admin-panel-content" class="rounded-lg shadow-lg flex-grow">
          <Route path="/admin-panel/general/*" component={General} />
          <Route path="/admin-panel/groups/*" component={Groups} />
          <Route path="/admin-panel/permissions/*" component={Permissions} />
          <Route path="/admin-panel/users/*" component={Users} />
          <Route path="/admin-panel/sessions/*" component={Sessions} />
          <Route path="/admin-panel" component={General} />
        </div>
      </div>
    </Router>
  </div>
</div>
