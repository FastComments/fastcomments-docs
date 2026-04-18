Aby zobaczyć wszystkie widgety i flow działające lokalnie dla publicznego tenanta `demo`, sklonuj repozytorium i uruchom:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Punkt wejścia aplikacji demonstracyjnej to `example/src/ShowcaseApp.tsx` — pojedyncza aplikacja, która udostępnia wszystkie widgety, motywy i flow.

Opcja `yarn web` używa `react-native-web` + `react-native-web-webview` (które renderuje WebView jako iframe). Przydatne do szybkich wizualnych testów typu smoke w przeglądarce; natywne API WebView, takie jak `injectJavaScript` i `onShouldStartLoadWithRequest`, nie będą w pełni działać na webie.