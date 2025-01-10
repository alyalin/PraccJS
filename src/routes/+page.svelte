<script lang="ts">
  import Tab from "../components/Tab.svelte";
  import Editor from "../components/Editor.svelte";
  import Result from "../components/Result.svelte";
  import { invoke } from "@tauri-apps/api/core";
    import { storageStore, updateTab } from "../stores/tabs";
    import { debounce } from "../utils/debounce";

  const debounceInvoke = debounce((tabId: string, content: string) => {
    const firstLine = content.slice(0, content.indexOf("\n"));
    updateTab(tabId, {content, name: firstLine ? firstLine : "New Tab"});
    invoke("handle_editor_changes", { sourceText: content, tabId });
  }, 500);

  async function onEditorModelChange(id: string, value: string) {
    debounceInvoke(id, value);
  }
</script>

<section class="flex flex-1 text-white">
  {#each $storageStore.tabs as tab (tab.id)}
    {#if tab.active}
      <Tab>
        <div class="flex flex-1 p-0 w-full" slot="content">
          <div class="flex w-6/12">
            <Editor
              onModelChange={onEditorModelChange}
              id={tab.id}
              content={tab.content}
            />
          </div>
          <div class="flex w-6/12">
            <Result result={tab.result} errors={tab.errors} />
          </div>
        </div>
      </Tab>
    {/if}
  {/each}
</section>
