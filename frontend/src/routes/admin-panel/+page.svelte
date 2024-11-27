<script>
    import Navbar from "$lib/layouts/navbar/Navbar.svelte";
    import Sidebar from "$lib/layouts/sidebar/Sidebar.svelte";
    import "./style.css";
</script>

<Navbar />
<div id="admin-panel-global">
    <Sidebar />
    <div id="admin-panel-content"></div>
</div>
