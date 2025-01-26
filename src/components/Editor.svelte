<script lang="ts">
    import type { ITab } from "../stores/tabs";
    import Monaco from "./Monaco.svelte";
    import { editor } from "monaco-editor";

    interface EditorProps extends Pick<ITab, 'id' | 'content'> {
        onModelChange: (id: string, value: string) => void,
    }

    let { onModelChange, id, content }: EditorProps =
        $props();
    
    function handleModelChange(content: string) {
        onModelChange(id, content);
    }

    function handleMonacoReady(editor: editor.IStandaloneCodeEditor) {
        $effect(() => {
            if (!editor.getValue().length) {
                editor.focus();
            }
        })
    }
</script>

<Monaco content={content} onModelChange={handleModelChange} onMonacoReady={handleMonacoReady} />
