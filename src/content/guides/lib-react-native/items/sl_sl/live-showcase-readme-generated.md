Za ogled vseh pripomočkov in tokov, ki tečejo lokalno proti javnemu `demo` najemniku, sklonirajte repozitorij in zaženite:

```bash
yarn bootstrap
cd example
yarn ios       # ali: yarn android, yarn web
```

Vstopna točka predstavitve je `example/src/ShowcaseApp.tsx` — ena aplikacija, ki prikaže vse pripomočke, teme in tokove.

Cilj `yarn web` uporablja `react-native-web` + `react-native-web-webview` (ki WebView upodobi kot iframe). Uporabno za hitre vizualne smoke teste v brskalniku; API-ji WebView, ki so na voljo samo v nativen okolju, kot so `injectJavaScript` in `onShouldStartLoadWithRequest`, se v brskalniku ne bodo v celoti obnašali.