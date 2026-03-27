This library is a complete react-native implementation of [FastComments](https://fastcomments.com).

It supports live commenting, chat, threads, emoticons, notifications, SSO, skins, and full customization by passing in a stylesheet object. All assets
can also be customized, and it supports toggling different assets based on dark mode.

The benefit of this library is that it is more flexible than the `fastcomments-react-native` wrapper. Comments are rendered with native components rather than inside a webview. Note: `react-native-webview` is still required as a transitive dependency of the rich text editor (`@10play/tentap-editor`).

It all runs on the FastComments backend, so you only have to incorporate the UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

See [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) for more examples.

Add live chat to your existing React Native application, or even build a social network!