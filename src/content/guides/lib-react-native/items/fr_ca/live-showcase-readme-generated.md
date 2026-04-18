Pour voir chaque widget et flux s'exécuter localement contre le locataire public `demo`, clonez le dépôt et exécutez :

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

The showcase entry point is `example/src/ShowcaseApp.tsx` — a single app that exposes all widgets, themes, and flows.

The `yarn web` target uses `react-native-web` + `react-native-web-webview` (which renders the WebView as an iframe). Useful for quick visual smoke tests in a browser; native-only WebView APIs like `injectJavaScript` and `onShouldStartLoadWithRequest` won't fully behave on web.