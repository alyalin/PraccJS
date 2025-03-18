<script lang="ts">
    import { tick } from "svelte";
    import type { ITab } from "../stores/tabs";
    import { platform } from "@tauri-apps/plugin-os";

    type TabsProps = {
        tabs: ITab[],
        addTab: () => void,
        removeTab: (tabId: string, tabs: ITab[]) => void,
        activateTab: (tabId: string) => void
    }

    const currentPlatform = platform();
    let { tabs, addTab, removeTab, activateTab }: TabsProps = $props();
    let tabsScrollContainer: HTMLElement;
    
    function handleAddTab() {
        addTab();

        setTimeout(() => {
            tabsScrollContainer.scrollLeft = tabsScrollContainer.scrollWidth;
        }, 0);
    }

    $effect.pre(() => {
        tick().then(() => {
            tabsScrollContainer.childNodes.forEach((tab: HTMLElement) => {
                if (tab?.classList?.contains('tab-active')) {
                    tab.scrollIntoView({ block: 'start', inline: 'start' });
                }
            })
        })
    })

</script>

<nav
    class="flex flex-1 items-center overflow-x-auto hide-scroll overflow-y-hidden"
    data-tauri-drag-region={ currentPlatform === "macos" || null }
>
    <ul class="flex flex-nowrap flex-grow-0 hide-scroll relative overflow-x-auto text-sm space-x-1 font-medium p-1" bind:this={tabsScrollContainer}>
        {#each tabs as tab (tab.id)}
            <li class={`relative group flex-shrink-0 ${
                        tab.active
                            ? "tab-active"
                            : ""
                    }`}>
                <button
                    class={`flex items-center px-3 rounded-sm max-w-32 flex-shrink-0 pr-10 py-1.5 ${
                        tab.active
                            ? "bg-muted"
                            : "group-hover:bg-secondary/80"
                    }`}
                    onclick={() => activateTab(tab.id)}
                >
                    <span class="whitespace-nowrap overflow-hidden text-clip">{tab.name}</span>
                </button>
                {#if tabs.length > 1}
                    <button
                        type="button"
                        aria-label="remove"
                        class={`${tab.active ? "visible hover:bg-background/30": "group-hover:visible invisible hover:bg-secondary/80"} bg-secondary rounded-md text-secondary-foreground flex items-center justify-center absolute inset-y-0 my-auto right-2 w-6 h-6 cursor-pointer`}
                        onclick={(e) => removeTab(tab.id, tabs)}
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="size-3"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M6 18 18 6M6 6l12 12"
                            />
                        </svg>
                    </button>
                {/if}
            </li>
        {/each}
    </ul>

    <ul class="flex items-center px-4 h-full" data-tauri-drag-region>
        <li class="flex-shrink-0">
            <button
            type="button"
            aria-label="add-new-tab"
            class="flex items-center justify-center w-6 h-6 font-semibold bg-secondary rounded-md text-secondary-foreground hover:bg-secondary/80"
            onclick={handleAddTab}
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-4"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M12 4.5v15m7.5-7.5h-15"
                />
            </svg>
        </button>
        </li>
    </ul>
</nav>

<style>

.hide-scroll {
    scrollbar-width: none;
}

.hide-scroll::-webkit-scrollbar {
    scrollbar-width: none;
    width: 0;
    height: 0;
}
</style>