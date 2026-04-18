Чтобы увидеть все виджеты и потоки, работающие локально с публичным тенантом `demo`, клонируйте репозиторий и выполните:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Точка входа демонстрации — `example/src/ShowcaseApp.tsx` — единое приложение, которое предоставляет все виджеты, темы и потоки.

Цель `yarn web` использует `react-native-web` + `react-native-web-webview` (которые рендерят WebView как iframe). Полезно для быстрых визуальных smoke-тестов в браузере; нативные API WebView, такие как `injectJavaScript` и `onShouldStartLoadWithRequest`, не будут полностью работать в вебе.