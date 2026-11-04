아바타, 중첩된 답글, 투표, 내장 리치 텍스트 작성기를 갖춘 실시간 스레드 댓글 기능에 더해 다크 테마와 실시간 채팅 프리셋을 제공합니다(여기서는 `react-native-web`을 통해 렌더링된 모습이 표시됨):

<table>
  <tr>
    <td align="center"><b>실시간 댓글</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="실시간 댓글, 라이트 테마"/></td>
    <td align="center"><b>다크 테마</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="실시간 댓글, 다크 테마"/></td>
    <td align="center"><b>실시간 채팅</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="실시간 채팅 프리셋"/></td>
  </tr>
</table>

### 리치 텍스트 에디터

이 라이브러리는 리치 텍스트 편집을 위해 [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html)를 사용하며, 강력한 WYSIWYG 편집 경험을 제공합니다. 동일한 에디터가 iOS, Android, 웹(`react-native-web` 경유)에서 모두 작동하므로, 단일 구현으로 모든 플랫폼에서 작성기가 일관되게 동작합니다.

`react-native-enriched`는 네이티브에서 React Native New Architecture(Fabric)를 필요로 합니다(RN 0.76부터 기본값, RN 0.72-0.75에서는 옵트인). 또한 패키지 `exports` 조건을 해석하는 번들러가 필요합니다. 이 SDK는 RN 0.81 / React 19 기준으로 개발 및 테스트되었습니다. 동일한 에디터는 `react-native-web`을 통해 웹에서도 실행되며, enriched 에디터의 웹 빌드는 업스트림에서 아직 실험적(experimental)으로 표시되어 있습니다.

### 위젯

SDK는 FastComments Android SDK와 동일하게 세 가지 위젯을 제공합니다:

- `FastCommentsLiveCommenting` - 투표, 답글, 페이지네이션, 멘션, 알림, 실시간 업데이트를 갖춘 스레드 댓글 기능.
- `FastCommentsLiveChat` - 동일한 엔진 위의 채팅 프리셋: 시간순 메시지(새 메시지는 하단에), 목록 아래의 작성기, 실시간 헤더 스트립(연결 상태 점 + 사용자 수), 위로 스크롤하여 로드되는 무한 히스토리, 새 메시지로 자동 스크롤, 투표 및 답글 스레딩 없음. 모든 프리셋은 `config`를 통해 재정의할 수 있습니다.
- `FastCommentsFeed` - 게시물 작성기, 미디어, 반응, 팔로우, 실시간 새 게시물 배너를 갖춘 소셜 피드.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### 테마

기본 외관은 일련의 시맨틱 디자인 토큰(`FastCommentsTheme`)에서 생성됩니다: 색상, 간격, 반경, 글꼴 크기, 글꼴 굵기, 아바타 크기. 부분 토큰 오버라이드(타입 `FastCommentsThemeOverrides`)를 어떤 위젯의 `theme` prop으로 전달하면 전체 스타일 트리가 일관되게 재스타일링됩니다:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

다크 모드는 토큰 세트 하나만 바꾸면 됩니다:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop은 여전히 세밀한 제어를 위한 원시 `IFastCommentsStyles` 트리를 받습니다. `theme`과 `styles`가 모두 제공되면 명시적 스타일이 테마 트리보다 우선합니다; `styles`만 제공되면 기본값을 완전히 대체합니다(원래 동작이므로 기존 통합 및 스킨은 영향을 받지 않습니다). `setupDarkModeSkin`은 `theme` prop으로 대체되어 더 이상 사용되지 않습니다.

### 설정 옵션

이 라이브러리는 웹 구현과 마찬가지로 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts)에 정의된 모든 설정 옵션을 지원하는 것을 목표로 합니다.

그 외에도 React Native는 `FastCommentsRNConfig`를 통해 몇 가지 SDK 전용 옵션을 추가합니다:

- `hideTopBar` - 작성기 위에 표시되는 로그인 사용자 / 알림 벨 스트립을 숨깁니다.
- `usePressToEdit` - 댓글을 길게 눌러 메뉴를 엽니다.
- `disableDownVoting` - 싫어요(다운보트) 버튼을 숨깁니다.
- `renderCommentInline` - 댓글 작성자 정보를 댓글 내용과 동일한 HTML 블록 안에 렌더링합니다.
- `renderLikesToRight` - 투표/좋아요 영역을 댓글 아래 대신 오른쪽으로 이동합니다.
- `renderDateBelowComment` - 날짜를 댓글 아래에 렌더링합니다.
- `showLiveStatus` - 댓글 위에 채팅 스타일의 "Live" + 사용자 수 헤더 스트립을 표시합니다.
- `useInlineSubmitButton` - 제출 버튼을 작성기 내부의 아이콘으로 렌더링합니다.
- `countAboveToggle` - `useShowCommentsToggle` 사용 시, "Show Comments" 토글 위에 렌더링할 댓글 수.
- `preserveFeedScrollPosition` - `FastCommentsFeed`가 언마운트/리마운트 간에 스크롤 위치를 기억합니다(기본값 true).

### FastComments 개념

시작하기 위해 알아야 할 주요 개념은 `tenantId`와 `urlId`입니다. `tenantId`는 FastComments.com 계정 식별자입니다. `urlId`는 댓글 스레드가 연결될 대상입니다. 페이지 URL, 제품 ID, 아티클 ID 등이 될 수 있습니다.

### 현지화

이 위젯들의 모든 사용자 대상 텍스트(버튼 라벨, 플레이스홀더, 빈 상태, "5분 전" 같은 상대 날짜, 오류 메시지 등)는 **서버 주도(server-driven)**입니다. 컴포넌트는 영어 문자열을 하드코딩하지 않으며, 요청된 로캘에 대해 FastComments가 제공하는 번역을 렌더링합니다.

로캘을 요청하려면 config에서 `locale`을 설정하세요:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es 등
};
```

`locale`이 설정되지 않으면 FastComments는 테넌트의 기본 언어를 제공합니다.

**텍스트 편집:** 번역은 이 SDK가 아닌 FastComments 대시보드에서 관리됩니다. 문구를 변경하거나 기본 문구를 재정의하거나 언어를 추가하려면 대시보드에서 계정의 번역을 편집하세요 - 변경 사항은 앱 릴리스 없이 위젯에 자동으로 반영됩니다. SDK는 영어 폴백을 포함하지 않으므로, 대시보드에서 키를 비우면 해당 항목은 빈 값으로 렌더링됩니다; 지원하는 모든 로캘에 대해 키를 채워 두세요.

### 사용자 알림

FastComments는 [다양한 시나리오](https://docs.fastcomments.com/guide-notifications.html)에 대한 알림을 지원합니다. 알림은 설정 가능하며, 전역 또는 알림/댓글 수준에서 옵트아웃할 수 있고, 사용자가 특정 페이지나 아티클의 스레드를 구독할 수 있는 페이지 수준 구독을 지원합니다.

예를 들어, Secure SSO를 사용해 사용자를 인증한 후 읽지 않은 알림을 주기적으로 폴링하여 사용자에게 푸시할 수 있습니다.

읽지 않은 사용자 알림을 가져오고 변환하는 방법은 [AppNotificationSecureSSO 예제](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)를 참고하세요.

### Gif 브라우저

기본적으로 이미지나 gif 선택은 활성화되어 있지 않습니다. 이미지 및 gif 업로드를 지원하는 방법은 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx)를 참고하세요. 이 라이브러리에는 검색과 제공된 이미지를 익명화하는 Gif 브라우저가 있으며, 사용하기만 하면 됩니다.

### 성능

성능 문제를 발견하면 사용한 기기를 포함하여 재현 가능한 예제와 함께 티켓을 열어 주세요. 성능은 모든 FastComments 라이브러리의 최우선 과제입니다.