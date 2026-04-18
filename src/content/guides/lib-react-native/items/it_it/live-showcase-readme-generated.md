Per vedere tutti i widget e i flussi in esecuzione localmente contro il tenant pubblico `demo`, clona il repo ed esegui:

```bash
yarn bootstrap
cd example
yarn ios       # oppure: yarn android, yarn web
```

Il punto d'ingresso del showcase è `example/src/ShowcaseApp.tsx` — un'unica app che espone tutti i widget, i temi e i flussi.

Il target `yarn web` usa `react-native-web` + `react-native-web-webview` (che renderizza la WebView come un iframe). Utile per rapidi test visivi di base in un browser; le API WebView disponibili solo su nativo come `injectJavaScript` e `onShouldStartLoadWithRequest` non si comporteranno pienamente sul web.