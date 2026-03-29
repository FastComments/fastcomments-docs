---
이 라이브러리는 [FastComments](https://fastcomments.com)에 대한 완전한 react-native 구현체입니다.

실시간 댓글(live commenting), 채팅(chat), 스레드(threads), 이모티콘(emoticons), 알림(notifications), SSO, 스킨(skins)을 지원하며 스타일시트 객체(stylesheet object)를 전달하여 완전한 커스터마이징이 가능합니다. 모든 자산(assets)도 사용자화할 수 있으며, 다크 모드에 따라 서로 다른 자산을 전환하는 기능을 지원합니다.

이 라이브러리의 장점은 `fastcomments-react-native` 래퍼보다 더 유연하다는 점입니다. 댓글은 웹뷰 내부가 아니라 네이티브 컴포넌트로 렌더링됩니다. 참고: 리치 텍스트 에디터(`@10play/tentap-editor`)의 전이적 종속성으로 `react-native-webview`가 여전히 필요합니다.

모든 것은 FastComments 백엔드에서 실행되므로 UI만 통합하면 됩니다:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

자세한 예시는 [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) 를 참조하세요.

기존 React Native 애플리케이션에 실시간 채팅을 추가하거나, 심지어 소셜 네트워크를 구축해 보세요!
---