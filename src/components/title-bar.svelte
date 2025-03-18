<script lang="ts">
    import { platform } from '@tauri-apps/plugin-os';
    const currentPlatform = platform();
</script>
 
<header class="flex">
    { #if currentPlatform === "macos" }
        <div data-tauri-drag-region class="w-28 flex-shrink-0"></div>
    {/if}
    <slot name="content"></slot>
</header>
