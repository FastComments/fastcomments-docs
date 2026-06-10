이 라이브러리는 [FastComments](https://fastcomments.com)의 완전한 react-native 구현입니다.

It supports live commenting, chat, threads, emoticons, notifications, SSO, skins, and full customization by passing in a stylesheet object. 모든 에셋
도 사용자화할 수 있으며, 다크 모드에 따라 서로 다른 에셋을 전환하는 것도 지원합니다.

이 라이브러리의 장점은 `fastcomments-react-native` 래퍼보다 더 유연하다는 것입니다. 댓글은 웹뷰 안에 렌더링되는 대신 네이티브 컴포넌트로 렌더링됩니다.

모든 것은 FastComments 백엔드에서 실행되므로 UI만 통합하면 됩니다:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

더 많은 예제는 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) 를 참조하세요.

기존 React Native 애플리케이션에 라이브 채팅을 추가하거나, 심지어 소셜 네트워크를 구축하세요!