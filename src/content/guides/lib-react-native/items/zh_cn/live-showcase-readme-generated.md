要查看针对公共 `demo` 租户在本地运行的每个小部件和流程，请克隆仓库并运行：

```bash
yarn bootstrap
cd example
yarn ios       # 或：yarn android, yarn web
```

The showcase entry point is `example/src/ShowcaseApp.tsx` — a single app that exposes all widgets, themes, and flows.

The `yarn web` target uses `react-native-web` + `react-native-web-webview` (which renders the WebView as an iframe). Useful for quick visual smoke tests in a browser; native-only WebView APIs like `injectJavaScript` and `onShouldStartLoadWithRequest` won't fully behave on web.