此库是对 [FastComments](https://fastcomments.com) 的完整 react-native 实现。

它支持实时评论、聊天、线程、表情符号、通知、SSO、皮肤，并且可以通过传入样式表对象进行完全自定义。所有资源也可以自定义，并且支持根据暗色模式切换不同资源。

此库的优点是比 `fastcomments-react-native` 包装器更灵活。评论使用原生组件呈现，而不是在 webview 内呈现。注意：`react-native-webview` 仍然作为富文本编辑器（`@10play/tentap-editor`）的传递依赖项被需要。

所有这些都在 FastComments 后端运行，所以您只需集成 UI：

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

有关更多示例，请参见 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src)。

将实时聊天添加到您现有的 React Native 应用，或甚至构建一个社交网络！