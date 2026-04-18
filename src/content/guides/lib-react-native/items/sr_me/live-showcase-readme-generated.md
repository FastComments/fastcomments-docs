Да бисте видјели сваки виџет и ток који се извршавају локално против јавног `demo` тенанта, клонирајте репозиторијум и покрените:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Улазна тачка за приказ је `example/src/ShowcaseApp.tsx` — једна апликација која омогућава приступ свим виџетима, темама и токовима.

Циљ `yarn web` користи `react-native-web` + `react-native-web-webview` (који рендерује WebView као iframe). Корисно за брзе визуелне провјере у прегледачу; WebView API-ји који су намењени само нативним платформама, као што су `injectJavaScript` и `onShouldStartLoadWithRequest`, неће се у потпуности понашати на вебу.