For at se alle widgets og flows kørende lokalt mod den offentlige `demo` tenant, klon repositoriet og kør:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Showcase-entrépunktet er `example/src/ShowcaseApp.tsx` — en enkelt app, der eksponerer alle widgets, temaer og flows.

Målet `yarn web` bruger `react-native-web` + `react-native-web-webview` (som gengiver WebView som en iframe). Nyttigt til hurtige visuelle smoke-tests i en browser; native-only WebView APIs som `injectJavaScript` og `onShouldStartLoadWithRequest` vil ikke opføre sig fuldt ud på web.