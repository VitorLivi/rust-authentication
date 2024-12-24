<script context="module">
    export type BreadcrumbItem = {
        name: string;
        path: string;
        children?: BreadcrumbItem[];
    };
</script>

<script lang="ts">
    import { Link, useLocation, type LOCATION } from "svelte-routing";

    export let location = useLocation();
    export let breadcrumbs: BreadcrumbItem[];

    const getCurrentBreadcrumbPath = (
        reactiveLocation: LOCATION,
    ): Omit<BreadcrumbItem, "children">[] | undefined => {
        let splitedPath = reactiveLocation.pathname.split("/") ?? [];

        splitedPath.shift();

        let result: Omit<BreadcrumbItem, "children">[] = [];
        let index = 0;
        let searchBreadcrumbs = [...breadcrumbs];
        while (searchBreadcrumbs.length && index < splitedPath.length) {
            const currentPath = splitedPath[index];

            const currentBreadcrumb = searchBreadcrumbs.find(
                (breadcrumb) => breadcrumb.path === currentPath,
            );

            if (!currentBreadcrumb) {
                return undefined;
            }

            result.push({
                name: currentBreadcrumb.name,
                path: currentBreadcrumb.path,
            });

            searchBreadcrumbs = currentBreadcrumb.children || [];
            index++;
        }

        return result;
    };

    export let currentBreadcrumbPath: Omit<BreadcrumbItem, "children">[] = [];
    $: {
        if (location && $location.pathname) {
            currentBreadcrumbPath = getCurrentBreadcrumbPath($location) ?? [];
        }
    }
</script>

<nav
    class="flex items-center space-x-2 text-sm text-gray-500 p-4 bg-gray-100 w-full"
>
    {#each currentBreadcrumbPath as breadcrumb, index}
        <div
            class:font-bold={index === 0}
            class="hover:text-blue-600 transition-colors"
        >
            <Link
                to={currentBreadcrumbPath.reduce(
                    (acc, current, currentIndex) =>
                        currentIndex <= index ? `${acc}/${current.path}` : acc,
                    "",
                )}
            >
                {breadcrumb.name}
            </Link>
        </div>

        {#if index !== 0}
            <span>/</span>
        {/if}
    {/each}
</nav>
