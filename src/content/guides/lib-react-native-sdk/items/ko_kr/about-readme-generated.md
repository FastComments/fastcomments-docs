이 라이브러리는 [FastComments](https://fastcomments.com)의 완전한 react-native 구현입니다.

It supports live commenting, chat, threads, emoticons, notifications, SSO, skins, and full customization by passing in a stylesheet object. All assets
또한 모든 에셋을 커스터마이즈할 수 있으며, 다크 모드에 따라 서로 다른 에셋을 전환하는 것을 지원합니다.

이 라이브러리의 장점은 더 유연하며, `fastcomments-react-native` 래퍼처럼 webview가 필요하지 않다는 점입니다.

모두 FastComments 백엔드에서 실행되므로, UI만 통합하면 됩니다:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

자세한 예제는 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) 를 참조하세요.

기존 React Native 애플리케이션에 라이브 채팅을 추가하거나, 심지어 소셜 네트워크를 구축해보세요!