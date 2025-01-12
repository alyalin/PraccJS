<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { editor, languages, Uri } from "monaco-editor";
    
    // import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
    import Dracula from '../assets/Dracula.json';


    let monacoHTMLElement: HTMLDivElement;
    let monacoEditor: editor.IStandaloneCodeEditor;

    interface MonacoProps {
        content: string,
        onModelChange?: (value: string) => void,
        readonly?: boolean,
        onMonacoReady?: (editor: editor.IStandaloneCodeEditor) => void
    }

    let { onModelChange, content, readonly, onMonacoReady }: MonacoProps =
        $props();

    // self.MonacoEnvironment = {
    //     getWorker: function (_: any, label: string) {
    //         return new tsWorker();
    //     },
    // };

    // languages.typescript.typescriptDefaults.setCompilerOptions({
    //     removeComments: false,
    //     pretty: true,
    // });
    // languages.typescript.typescriptDefaults.setEagerModelSync(true);
    // languages.typescript.typescriptDefaults.setDiagnosticsOptions({
    //     noSemanticValidation: true,
    //     noSyntaxValidation: true,
    // });

    editor.defineTheme('dracula', {
        base: 'vs-dark',
        inherit: true,
        ...Dracula
    });


    onMount(async () => {

        monacoEditor = editor.create(monacoHTMLElement, {
            language: "javascript",
            automaticLayout: true,
            theme: 'dracula',
            readOnly: readonly ? readonly : false,
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
            scrollbar: {
                horizontal: "hidden",
            },
            glyphMargin: false,
            lineNumbersMinChars: 0,
        });


        if (onMonacoReady) {
            onMonacoReady(monacoEditor);
        }


        // const uri = monacoEditor.getModel().uri;

        // console.log(monacoEditor.getModel().uri);

        // const worker = await languages.typescript.getTypeScriptWorker();
        // const client = await worker(uri);
        // const result = await client.getEmitOutput(uri.toString());
        

        monacoEditor.getModel().onDidChangeContent((event) => {
            if (onModelChange) {
                onModelChange(monacoEditor.getValue());
            }
        });
    });

    onDestroy(() => {
        editor.getModels().forEach((model) => model.dispose());
        monacoEditor?.dispose();
    });

</script>

<div class="flex-1" bind:this={monacoHTMLElement}></div>