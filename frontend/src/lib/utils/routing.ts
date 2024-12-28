import type { LOCATION } from "svelte-routing";

export const isRoute = (route: string, location: LOCATION) => {
  const path = location.pathname;
  return path === route;
}
