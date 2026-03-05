---
这个库是 [FastComments](https://fastcomments.com) 的完整 react-native 实现。

它支持实时评论、聊天、线程、表情符号、通知、SSO、皮肤，并且可以通过传入一个 stylesheet 对象进行完全自定义。所有资产也可以自定义，并且支持根据深色模式切换不同的资产。

这个库的好处是更灵活，并且不像 `fastcomments-react-native` 包装器那样需要使用 webview。

这些功能全部运行在 FastComments 后端，因此你只需集成 UI：

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

更多示例请参见 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src)。

将实时聊天添加到你现有的 React Native 应用，甚至构建一个社交网络！
---