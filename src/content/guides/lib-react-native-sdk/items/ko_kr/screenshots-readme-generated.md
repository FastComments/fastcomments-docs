#### 스킨: Erebus
![스킨: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### 스킨: Default
![스킨: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### 이미지 지원 네이티브 WYSIWYG 편집기!
![이미지 지원 네이티브 WYSIWYG 편집기](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### 리치 텍스트 편집기

이 라이브러리는 강력한 WYSIWYG 편집 경험을 제공하는 10tap 에디터를 리치 텍스트 편집 기능으로 사용합니다.

### 구성 옵션

이 라이브러리는 웹 구현과 마찬가지로 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts)에 정의된 모든 구성 옵션을 지원하는 것을 목표로 합니다.

### FastComments 개념

시작하기 위해 알아야 할 주요 개념은 `tenantId`와 `urlId`입니다. `tenantId`는 FastComments.com 계정 식별자입니다. `urlId`는 댓글 스레드가 연결될 위치입니다. 이는 페이지 URL, 제품 ID, 기사 ID 등일 수 있습니다.

### 사용자 알림

FastComments는 [다양한 시나리오](https://docs.fastcomments.com/guide-notifications.html)에 대한 알림을 지원합니다. 알림은 구성 가능하며, 전역 또는 알림/댓글 수준에서 수신 거부할 수 있고, 페이지 수준 구독을 지원하여 사용자가 특정 페이지나 기사 스레드에 구독할 수 있습니다.

예를 들어, Secure SSO를 사용하여 사용자를 인증한 다음 정기적으로 읽지 않은 알림을 폴링하고 이를 사용자에게 푸시할 수 있습니다.

읽지 않은 사용자 알림을 가져오고 변환하는 방법은 [예제 AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)를 참조하세요.

### GIF 브라우저

기본적으로 이미지나 GIF 선택은 활성화되어 있지 않습니다. 이미지 및 GIF 업로드를 지원하는 방법은 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx)를 참조하세요. 이 라이브러리에는 검색 및 제공된 이미지를 익명화하는 GIF 브라우저가 포함되어 있으니 이를 사용하면 됩니다.

### 성능

성능 문제가 발견되면 재현 가능한 예제(사용된 기기 포함)를 첨부하여 티켓을 열어주세요. 성능은 모든 FastComments 라이브러리에서 최우선 사항입니다.