export const adminPanelBreadcrumbs = [
  {
    name: "Admin Panel",
    path: "admin-panel",
    children: [
      {
        name: "General",
        path: "general",
        children: [],
      },
      {
        name: "Groups",
        path: "groups",
        children: [
          {
            name: "Add Group",
            path: "add-group",
          },
        ],
      },
      {
        name: "Permissions",
        path: "permissions",
        children: [
          {
            name: "Add Permission",
            path: "add-permission",
          },
        ],
      },
      {
        name: "Users",
        path: "users",
        children: [
          {
            name: "Add User",
            path: "add-user",
          },
          {
            name: "Edit User",
            path: "edit-user",
          },
        ],
      },
      {
        name: "Sessions",
        path: "sessions",
        children: [],
      },
    ],
  },
]
