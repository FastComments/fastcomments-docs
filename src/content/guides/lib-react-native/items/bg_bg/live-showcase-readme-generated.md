За да видите всеки уиджет и поток, работещи локално срещу публичния наемател `demo`, клонирайте хранилището и изпълнете:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Входната точка на демонстрацията е `example/src/ShowcaseApp.tsx` — едно приложение, което показва всички уиджети, теми и потоци.

Таргетът `yarn web` използва `react-native-web` + `react-native-web-webview` (което рендерира WebView като iframe). Полезно за бързи визуални тестове в браузър; native-only WebView APIs като `injectJavaScript` и `onShouldStartLoadWithRequest` няма да се държат напълно еднакво в уеб.