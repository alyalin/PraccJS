<script lang="ts">
    import type { ITab } from "../stores/tabs";
    import Monaco from "./Monaco.svelte";

    interface EditorProps extends Pick<ITab, 'id' | 'content'> {
        onModelChange: (id: string, value: string) => void,
    }

    let { onModelChange, id, content }: EditorProps =
        $props();

    
    function handleModelChange(content: string) {
        onModelChange(id, content);
    }
</script>

<Monaco content={content} onModelChange={handleModelChange} />
