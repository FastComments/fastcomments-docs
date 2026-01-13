此函式庫是 [FastComments](https://fastcomments.com) 的完整 react-native 實作。

它支援即時留言、聊天、討論串、表情符號、通知、SSO、主題，以及透過傳入樣式表物件進行完全自訂。所有資源也可以自訂，並支援根據深色模式切換不同資源。

此函式庫的好處是更具彈性，且不像 `fastcomments-react-native` wrapper 那樣需要使用 webview。

所有運作都在 FastComments 後端，因此你只需要整合使用者介面：

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

更多範例請參閱 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src)。

將即時聊天加入您現有的 React Native 應用程式，或甚至建立一個社群網路！