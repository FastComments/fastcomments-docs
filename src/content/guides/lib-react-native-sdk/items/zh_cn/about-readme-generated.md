该库是 [FastComments](https://fastcomments.com) 的一个完整的 react-native 实现。

它支持实时评论、聊天、线程、表情、通知、SSO、皮肤，并且可以通过传入样式表对象实现全面定制。所有资源也可以自定义，并且支持根据暗色模式切换不同的资源。

该库的优势是比 `fastcomments-react-native` 包装器更灵活。评论使用原生组件渲染，而不是在 webview 中。

这一切都运行在 FastComments 后端，因此你只需集成 UI：

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

更多示例请参见 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src)。

将实时聊天添加到您现有的 React Native 应用，甚至构建一个社交网络！