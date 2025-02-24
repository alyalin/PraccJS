<script lang="ts">
    import { tick } from "svelte";
    import type { ITab } from "../stores/tabs";
    import Monaco from "./Monaco.svelte";
    import { editor } from "monaco-editor";

    interface EditorProps extends Pick<ITab, 'id' | 'content'> {
        onModelChange: (id: string, value: string) => void,
    }

    let editorRef;

    let { onModelChange, id, content }: EditorProps =
        $props();
    
    function handleModelChange(content: string) {
        onModelChange(id, content);
    }

    function handleMonacoReady(editor: editor.IStandaloneCodeEditor) {
        editorRef = editor;
        $effect(() => {
            if (!editor.getValue().length) {
                editor.focus();
            }
        })
    }

    $effect.pre(() => {
        tick().then(() => {
            editorRef.focus();
        })
    })
</script>

<Monaco content={content} onModelChange={handleModelChange} onMonacoReady={handleMonacoReady} />
