Om elke widget en flow lokaal tegen de publieke `demo` tenant te zien draaien, kloon de repo en voer het volgende uit:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Het toegangspunt van de showcase is `example/src/ShowcaseApp.tsx` — een enkele app die alle widgets, thema's en flows beschikbaar maakt.

Het `yarn web`-doel gebruikt `react-native-web` + `react-native-web-webview` (die de WebView als een iframe rendert). Handig voor snelle visuele smoke-tests in een browser; native-only WebView-API's zoals `injectJavaScript` en `onShouldStartLoadWithRequest` zullen op het web niet volledig werken.