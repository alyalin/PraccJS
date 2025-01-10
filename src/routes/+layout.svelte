<script lang="ts">
	import "../app.css";
	import TitleBar from "../components/title-bar.svelte";
	import Tabs from "../components/Tabs.svelte";
    import { activateTab, addTab, removeTab, storageStore } from "../stores/tabs";

</script>

<main class="flex flex-col h-dvh">
	<TitleBar>
		<Tabs slot="content" tabs={$storageStore.tabs} addTab={addTab} activateTab={activateTab} removeTab={removeTab} />
	</TitleBar>
	
	<slot />
</main>
