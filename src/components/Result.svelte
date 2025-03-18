<script lang="ts">
    import { editor } from "monaco-editor";
    import Monaco from "./Monaco.svelte";

    type ResultProps = {
        result: string,
        errors: string
    }

    let { result, errors }: ResultProps = $props();

    function handleMonacoReady(editor: editor.IStandaloneCodeEditor) {
        $effect(() => {
            editor.setValue(result || errors || "");
        })
    }

</script>

<Monaco content={result || errors || ""} readonly={true} onMonacoReady={handleMonacoReady} />