Para ver todos los widgets y flujos ejecutándose localmente contra el tenant público `demo`, clona el repositorio y ejecuta:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

El punto de entrada de la demostración es `example/src/ShowcaseApp.tsx` — una única aplicación que expone todos los widgets, temas y flujos.

El objetivo `yarn web` utiliza `react-native-web` + `react-native-web-webview` (que renderiza el WebView como un iframe). Útil para pruebas rápidas de humo visual en un navegador; las APIs de WebView exclusivas de nativo como `injectJavaScript` y `onShouldStartLoadWithRequest` no se comportarán completamente en la web.