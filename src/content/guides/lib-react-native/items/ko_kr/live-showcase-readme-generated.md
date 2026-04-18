---
공용 `demo` 테넌트에 대해 로컬에서 실행되는 모든 위젯과 플로우를 보려면, 저장소를 클론하고 다음을 실행하세요:

```bash
yarn bootstrap
cd example
yarn ios       # 또는: yarn android, yarn web
```

The showcase entry point is `example/src/ShowcaseApp.tsx` — a single app that exposes all widgets, themes, and flows.

The `yarn web` target uses `react-native-web` + `react-native-web-webview` (which renders the WebView as an iframe). Useful for quick visual smoke tests in a browser; native-only WebView APIs like `injectJavaScript` and `onShouldStartLoadWithRequest` won't fully behave on web.
---