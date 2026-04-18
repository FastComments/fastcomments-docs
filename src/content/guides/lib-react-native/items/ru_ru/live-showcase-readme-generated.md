Чтобы увидеть все виджеты и потоки, запускающиеся локально против публичного тенанта `demo`, клонируйте репозиторий и выполните:

```bash
yarn bootstrap
cd example
yarn ios       # или: yarn android, yarn web
```

Точка входа демонстрации — `example/src/ShowcaseApp.tsx` — единое приложение, которое предоставляет все виджеты, темы и потоки.

Цель `yarn web` использует `react-native-web` + `react-native-web-webview` (которые рендерят WebView как iframe). Полезно для быстрых визуальных smoke-тестов в браузере; WebView API, доступные только в нативной среде, такие как `injectJavaScript` и `onShouldStartLoadWithRequest`, не будут полностью работать в вебе.