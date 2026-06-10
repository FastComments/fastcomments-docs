#### 스킨: Erebus
![스킨: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### 스킨: Default
![스킨: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### 이미지 지원 네이티브 WYSIWYG 편집기!
![이미지 지원 네이티브 WYSIWYG 편집기](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### 리치 텍스트 편집기

이 라이브러리는 강력한 WYSIWYG 편집 경험을 제공하는 [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched)을 리치 텍스트 편집에 사용합니다. 동일한 편집기가 iOS, Android 및 웹(`react-native-web`을 통해)을 모두 구동하므로, 단일 구현으로 모든 플랫폼에서 작성기가 일관되게 동작합니다.

`react-native-enriched`는 네이티브에서 React Native New Architecture(Fabric)를 요구하며, 패키지 `exports` 조건을 해결하는 번들러(Metro with package exports / RN 0.72+)가 필요합니다. 웹 지원은 현재 실험적입니다.

### 구성 옵션

이 라이브러리는 웹 구현과 마찬가지로 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts)에 정의된 모든 구성 옵션을 지원하는 것을 목표로 합니다.

### FastComments 개념

시작하기 위해 알아야 할 주요 개념은 `tenantId`와 `urlId`입니다. `tenantId`는 FastComments.com 계정 식별자입니다. `urlId`는 댓글 스레드가 연결될 위치입니다. 이 값은 페이지 URL, 제품 ID, 기사 ID 등일 수 있습니다.

### 사용자 알림

FastComments는 [다양한 시나리오](https://docs.fastcomments.com/guide-notifications.html)에 대한 알림을 지원합니다. 알림은 구성 가능하며, 전역적으로 또는 개별 알림/댓글 수준에서 수신 거부할 수 있고, 페이지 수준 구독을 지원하므로 사용자는 특정 페이지나 기사 스레드를 구독할 수 있습니다.

예를 들어, Secure SSO를 사용해 사용자를 인증한 다음 주기적으로 읽지 않은 알림을 폴링하여 사용자에게 푸시할 수 있습니다.

읽지 않은 사용자 알림을 가져오고 변환하는 방법은 [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)를 참조하세요.

### Gif 브라우저

기본적으로 이미지 또는 gif 선택은 활성화되어 있지 않습니다. 이미지 및 gif 업로드를 지원하는 방법은 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx)를 참조하세요. 이 라이브러리에는 검색 및 제공된 이미지를 익명화하는 Gif 브라우저가 포함되어 있으며, 단순히 이를 사용하면 됩니다.

### 성능

성능 문제를 확인한 경우, 사용한 기기 정보를 포함한 재현 가능한 예제를 첨부하여 티켓을 열어 주세요. 성능은 모든 FastComments 라이브러리에서 최우선 고려사항입니다.