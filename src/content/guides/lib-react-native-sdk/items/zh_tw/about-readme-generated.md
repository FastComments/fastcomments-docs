This library is a complete react-native implementation of [FastComments](https://fastcomments.com).

它支援即時評論、聊天、討論串、表情符號、通知、SSO、主題，並且可以透過傳入樣式表物件進行完全自訂。所有資源也都可以自訂，並支援依據深色模式切換不同資源。

此函式庫的優點在於比 `fastcomments-react-native` 封裝器更具彈性。評論是使用原生元件渲染，而不是在 webview 中。注意：`react-native-webview` 仍然是富文字編輯器 (`@10play/tentap-editor`) 的一個遞迴相依項。

這一切都在 FastComments 後端運行，所以你只需要整合 UI：

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

請參閱 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) 取得更多範例。

將即時聊天加入你現有的 React Native 應用程式，或甚至建立一個社群網路！