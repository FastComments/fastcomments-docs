Да бисте видели све видџете и токове који се покрећу локално против јавног `demo` тенанта, клонирајте репозиториј и покрените:

```bash
yarn bootstrap
cd example
yarn ios       # или: yarn android, yarn web
```

Улазна тачка приказа је `example/src/ShowcaseApp.tsx` — једна апликација која приказује све видџете, теме и токове.

Циљ `yarn web` користи `react-native-web` + `react-native-web-webview` (који рендерује WebView као iframe). Корисно за брзе визуелне провјере у прегледачу; WebView API-ји који су доступни само у нативном окружењу као `injectJavaScript` и `onShouldStartLoadWithRequest` неће у потпуности радити на вебу.