<script lang="ts">
    import { editor } from "monaco-editor";
    import { onDestroy, onMount } from "svelte";
    import type { ITab } from "../stores/tabs";

    let logger: editor.IStandaloneCodeEditor;
    let loggerElement: HTMLDivElement;

    interface ResultProps extends Pick<ITab, "result" | "errors"> {}

    let { result, errors }: ResultProps = $props();

    function handleResize() {
        logger.layout();
    }

    onMount(() => {
        logger = editor.create(loggerElement, {
            language: "typescript",
            automaticLayout: true,
            value: result,
            theme: "vs-dark",
            readOnly: true,
            minimap: {
                enabled: false,
            },
            scrollbar: {
                horizontal: "hidden",
            },
            lineHeight: 30,
            renderLineHighlight: "none",
            renderWhitespace: "none",
            cursorBlinking: "smooth",
            lineNumbers: "off",
            overviewRulerBorder: false,
            hideCursorInOverviewRuler: true,
            lineNumbersMinChars: 0,
            cursorWidth: 1,
            cursorStyle: "block-outline",
            cursorSmoothCaretAnimation: "off",
            fontSize: 16,
            // lineNumbers: "off",
            wordWrap: "on",
        });


        $effect(() => {
            console.log(errors);
            console.log(result);
            if (result) {
                logger.setValue(result);
            }

            if (errors) {
                logger.setValue(errors);
            }

            if (!result && !errors) {
                console.log("here empty", result, errors);
                logger.setValue("");
            }
        });
    });

    onDestroy(() => {
        editor.getModels().forEach((model) => model.dispose());
        logger.dispose();
    });
</script>

<svelte:window 
  on:resize={handleResize} 
  />

<div class="flex-1 min-w-0" bind:this={loggerElement}></div>
