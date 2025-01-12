<script lang="ts">
    import { editor } from "monaco-editor";
    import type { ITab } from "../stores/tabs";
    import Monaco from "./Monaco.svelte";

    interface ResultProps extends Pick<ITab, "result" | "errors"> {}

    let { result, errors }: ResultProps = $props();

    function handleMonacoReady(editor: editor.IStandaloneCodeEditor) {
        $effect(() => {
            editor.setValue(result || errors || "");
        })
    }

</script>

<Monaco content={result || errors || ""} readonly={true} onMonacoReady={handleMonacoReady} />