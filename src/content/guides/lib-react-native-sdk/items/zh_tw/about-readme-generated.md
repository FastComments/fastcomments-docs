這個函式庫是 [FastComments](https://fastcomments.com) 的完整 react-native 實作。

它支援即時留言、聊天、討論串、表情符號、通知、SSO、主題樣式（skins），並可透過傳入樣式表物件來進行完整客製化。所有資產
也可以自訂，並支援根據深色模式切換不同資產。

這個函式庫的好處是比 `fastcomments-react-native` 包裝器更具彈性。留言是以原生元件渲染，而不是在 webview 中。

它全部在 FastComments 後端運行，所以你只需要整合 UI：

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

更多範例請參見 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src)。

將即時聊天加入您現有的 React Native 應用程式，或甚至構建一個社群網路！