Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>실시간 댓글</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="실시간 댓글, 라이트 테마"/></td>
    <td align="center"><b>다크 테마</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="실시간 댓글, 다크 테마"/></td>
    <td align="center"><b>실시간 채팅</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="실시간 채팅 프리셋"/></td>
  </tr>
</table>

### Rich Text Editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widgets

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` - 투표, 답글, 페이지네이션, 멘션, 알림 및 실시간 업데이트가 포함된 스레드 댓글.
- `FastCommentsLiveChat` - 동일 엔진을 기반으로 한 채팅 프리셋: 최신 메시지가 아래에 표시되는 연대순 메시지, 리스트 아래에 위치한 컴포저, 실시간 헤더 스트립(연결 점 + 사용자 수), 위로 스크롤하여 로드되는 무한 히스토리, 새 메시지 자동 스크롤, 투표 및 답글 스레딩 없음. 모든 프리셋은 `config`를 통해 재정의할 수 있습니다.
- `FastCommentsFeed` - 포스트 컴포저, 미디어, 반응, 팔로우 및 실시간 새 포스트 배너가 포함된 소셜 피드.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

The default look is generated from a set of semantic design tokens (`FastCommentsTheme`): colors, spacing, radius, font sizes, font weights, and avatar sizes. Pass partial token overrides (typed `FastCommentsThemeOverrides`) through the `theme` prop on any widget and the entire style tree restyles consistently:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Dark mode is one token set away:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

The `styles` prop still accepts a raw `IFastCommentsStyles` tree for surgical control. When `theme` and `styles` are both provided, the explicit styles win over the themed tree; when only `styles` is provided, it replaces the defaults entirely (the original behavior, so existing integrations and skins are unaffected). `setupDarkModeSkin` is deprecated in favor of the `theme` prop.

### Configuration Options

This library aims to support all configuration options defined in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), just like the web implementation.

On top of those, React Native adds a few SDK-specific options via `FastCommentsRNConfig`:

- `hideTopBar` - 컴포저 위에 표시되는 로그인된 사용자 / 알림 벨 스트립을 숨깁니다.
- `usePressToEdit` - 댓글을 길게 눌러 메뉴를 엽니다.
- `disableDownVoting` - 다운투표 버튼을 숨깁니다.
- `renderCommentInline` - 댓글 내용과 같은 HTML 블록 안에 댓글 작성자 정보를 렌더링합니다.
- `renderLikesToRight` - 투표/좋아요 영역을 댓글 아래가 아니라 오른쪽으로 이동합니다.
- `renderDateBelowComment` - 날짜를 댓글 아래에 표시합니다.
- `showLiveStatus` - 댓글 위에 채팅 스타일의 "Live" + 사용자 수 헤더 스트립을 표시합니다.
- `useInlineSubmitButton` - 컴포저 안에 아이콘 형태의 전송 버튼을 렌더링합니다.
- `countAboveToggle` - `useShowCommentsToggle`와 함께 사용할 때, "Show Comments" 토글 위에 몇 개의 댓글을 표시할지 지정합니다.
- `preserveFeedScrollPosition` - `FastCommentsFeed`가 언마운트/리마운트 간에 스크롤 오프셋을 기억합니다(기본값 true).

### FastComments Concepts

시작하기 위해 알아야 할 주요 개념은 `tenantId`와 `urlId`입니다. `tenantId`는 FastComments.com 계정 식별자이며, `urlId`는 댓글 스레드가 연결될 위치를 나타냅니다. 페이지 URL, 제품 ID, 기사 ID 등 어떤 것이든 될 수 있습니다.

### Localization

이 위젯들의 모든 사용자 인터페이스 텍스트(버튼 라벨, 플레이스홀더, 빈 상태, "5 minutes ago"와 같은 상대 날짜, 오류 메시지 등)는 **서버 기반**입니다. 컴포넌트는 영어 문자열을 하드코딩하지 않으며, 요청된 로케일에 대해 FastComments가 제공하는 번역을 렌더링합니다.

로케일을 요청하려면 config에 `locale`을 설정하세요:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

`locale`이 설정되지 않으면 FastComments는 테넌트의 기본 언어를 제공합니다.

**텍스트 편집:** 번역은 이 SDK가 아니라 FastComments 대시보드에서 관리됩니다. 문구를 변경하려면 기본 복사본을 재정의하거나 언어를 추가하고, 대시보드에서 계정의 번역을 편집하면 위젯이 자동으로 변경을 반영합니다. 앱 릴리스를 별도로 할 필요가 없습니다. SDK는 영어 폴백을 제공하지 않으므로 대시보드에서 키를 비워두면 해당 로케일에서 빈 문자열이 표시됩니다; 지원하는 모든 로케일에 대해 키를 채워두세요.

### User Notifications

FastComments는 [많은 시나리오](https://docs.fastcomments.com/guide-notifications.html)에서 알림을 지원합니다. 알림은 구성 가능하며, 전역 또는 알림/댓글 수준에서 옵트아웃할 수 있고, 페이지 수준 구독을 지원하여 사용자가 특정 페이지나 기사 스레드에 구독할 수 있습니다.

예를 들어, Secure SSO를 사용해 사용자를 인증한 뒤 주기적으로 읽지 않은 알림을 폴링하고 사용자에게 푸시할 수 있습니다.

읽지 않은 사용자 알림을 가져오고 번역하는 방법은 [예제 AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)를 참고하세요.

### Gif Browser

기본적으로 이미지나 GIF 선택이 활성화되어 있지 않습니다. 이미지 및 GIF 업로드를 지원하는 방법은 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx)를 참고하세요. 이 라이브러리에는 검색 및 이미지를 익명화하는 Gif Browser가 포함되어 있으니, 이를 사용하면 됩니다.

### Performance

성능 문제가 발견되면 재현 가능한 예시와 사용한 디바이스 정보를 포함하여 티켓을 열어 주세요. 성능은 모든 FastComments 라이브러리에서 일등급으로 다루어집니다.