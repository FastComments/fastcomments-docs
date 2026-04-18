Um jedes Widget und jeden Flow lokal gegen den öffentlichen `demo` Tenant auszuführen, klone das Repository und führe aus:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Der Einstiegspunkt der Showcase ist `example/src/ShowcaseApp.tsx` — eine einzelne App, die alle Widgets, Themes und Flows bereitstellt.

Das `yarn web`-Ziel verwendet `react-native-web` + `react-native-web-webview` (welches die WebView als ein iframe rendert). Nützlich für schnelle visuelle Smoke-Tests im Browser; nur auf nativen Plattformen verfügbare WebView-APIs wie `injectJavaScript` und `onShouldStartLoadWithRequest` verhalten sich im Web nicht vollständig gleich.