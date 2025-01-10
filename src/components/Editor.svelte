<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    import { editor, languages } from "monaco-editor";
    
    import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
    import type { ITab } from "../stores/tabs";

    let editorElement: HTMLDivElement;
    let codeEditor: editor.IStandaloneCodeEditor;

    type Theme = editor.BuiltinTheme;
    let vsDark: Theme = 'vs-dark' as Theme;

    import Dracula from '../assets/Dracula.json';

    interface EditorProps extends Pick<ITab, 'id' | 'content'> {
        onModelChange: (id: string, value: string) => void,
    }

    let { onModelChange, id, content }: EditorProps =
        $props();

    editor.defineTheme('vs-dark', {
        base: 'vs-dark',
        inherit: true,
        ...Dracula
    });

    function handleResize() {
        codeEditor.layout();
    }

    onMount(async () => {
        self.MonacoEnvironment = {
            getWorker: function (_: any, label: string) {
                return new tsWorker();
            },
        };

        languages.typescript.typescriptDefaults.setEagerModelSync(true);
        languages.typescript.typescriptDefaults.setDiagnosticsOptions({
            // noSemanticValidation: true,
            // noSyntaxValidation: true,
        });


        codeEditor = editor.create(editorElement, {
            language: "typescript",
            automaticLayout: true,
            value: content,
            minimap: {
                enabled: false,
            },
            lineHeight: 30,
            renderLineHighlight: "none",
            renderWhitespace: "none",
            cursorBlinking: "smooth",
            cursorWidth: 1,
            cursorStyle: "block-outline",
            cursorSmoothCaretAnimation: "off",
            fontSize: 16,
            lineNumbers: "off",
            overviewRulerBorder: false,
            wordWrap: "on",
            hideCursorInOverviewRuler: true,
            overviewRulerLanes: 0,
            // hideCursorInOverviewRuler: true,
            scrollbar: {
                horizontal: "hidden",
            },
            // folding: false,
            glyphMargin: false,
            // lineDecorationsWidth: 0,
            lineNumbersMinChars: 0,
            // decorationsOverviewRuler: false,
        });

        codeEditor.getModel().onDidChangeContent((event) => {
            onModelChange(id, codeEditor.getValue());
        });
    });

    onDestroy(() => {
        editor.getModels().forEach((model) => model.dispose());
        codeEditor?.dispose();
    });
</script>

<svelte:window 
  on:resize={handleResize} 
  />

<div class="flex-1 min-w-0" bind:this={editorElement}></div>
