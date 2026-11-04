要在本地针对公共 `demo` 租户运行所有小部件和流程，请克隆仓库并运行：

```bash
yarn bootstrap
cd example
yarn ios       # 或: yarn android, yarn web
```

展示入口是 `example/src/ShowcaseApp.tsx` —— 一个暴露所有小部件、主题和流程的单一应用。

`yarn web` 目标使用 `react-native-web` + `react-native-web-webview`（它将 WebView 渲染为 iframe）。适用于在浏览器中进行快速的视觉冒烟测试；仅限原生的 WebView API（如 `injectJavaScript` 和 `onShouldStartLoadWithRequest`）在 web 上无法完全正常工作。