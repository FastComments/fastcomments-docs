要在本地查看針對公開 `demo` 租戶運行的所有元件和流程，請複製此倉庫並執行：

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

展示的進入點是 `example/src/ShowcaseApp.tsx` — 一個單一的應用程式，呈現所有元件、主題和流程。

`yarn web` 目標使用 `react-native-web` + `react-native-web-webview`（會將 WebView 呈現為 iframe）。這對於在瀏覽器中快速進行視覺性冒煙測試很有用；僅限原生的 WebView API（例如 `injectJavaScript` 和 `onShouldStartLoadWithRequest`）在網頁上不會完全如預期運作。