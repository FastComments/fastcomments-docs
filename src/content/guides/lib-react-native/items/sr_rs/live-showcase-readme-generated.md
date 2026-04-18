Да бисте видели сваки видгет и ток који се покреће локално против јавног `demo` tenant-а, клонирајте репозиторијум и покрените:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Улазна тачка демонстрације је `example/src/ShowcaseApp.tsx` — једна апликација која приказује све видгете, теме и токове.

Циљна платформа `yarn web` користи `react-native-web` + `react-native-web-webview` (који приказује WebView као iframe). Корисно за брзе визуелне smoke тестове у прегледачу; нативни WebView API-ји попут `injectJavaScript` и `onShouldStartLoadWithRequest` неће се у потпуности понашати на вебу.