Щоб побачити всі віджети та потоки, які працюють локально з публічним тенантом `demo`, склонуйте репозиторій і виконайте:

```bash
yarn bootstrap
cd example
yarn ios       # або: yarn android, yarn web
```

Точка входу демонстрації — `example/src/ShowcaseApp.tsx` — єдиний додаток, що показує всі віджети, теми та потоки.

Ціль `yarn web` використовує `react-native-web` + `react-native-web-webview` (який рендерить WebView як iframe). Корисно для швидких візуальних smoke-тестів у браузері; нативні API WebView, такі як `injectJavaScript` і `onShouldStartLoadWithRequest`, не будуть повністю працювати в веб-середовищі.