Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>실시간 댓글</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>다크 테마</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>라이브 채팅</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich Text Editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widgets

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` - 스레드형 댓글, 투표, 답글, 페이지네이션, 멘션, 알림 및 실시간 업데이트를 제공합니다.
- `FastCommentsLiveChat` - 동일한 엔진을 사용한 채팅 프리셋: 하단에 새로운 메시지가 쌓이는 연대순 메시지, 리스트 아래에 컴포저, 연결 상태 점 및 사용자 수가 표시되는 헤더 스트립, 위로 스크롤하여 무한히 로드되는 히스토리, 새 메시지 자동 스크롤, 투표나 답글 스레딩 없음. 모든 프리셋은 `config`를 통해 재정의할 수 있습니다.
- `FastCommentsFeed` - 포스트 컴포저, 미디어, 리액션, 팔로우 및 실시간 신규 포스트 배너가 포함된 소셜 피드.

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

- `hideTopBar` - 컴포저 위에 표시되는 로그인된 사용자 / 알림 벨 아이콘 스트립을 숨깁니다.
- `usePressToEdit` - 댓글을 길게 눌러 메뉴를 엽니다.
- `disableDownVoting` - 다운투표 버튼을 숨깁니다.
- `renderCommentInline` - 댓글 내용을 포함하는 동일 HTML 블록 안에 댓글 작성자 정보를 렌더링합니다.
- `renderLikesToRight` - 투표/좋아요 영역을 댓글 아래가 아니라 오른쪽으로 이동합니다.
- `renderDateBelowComment` - 댓글 아래에 날짜를 표시합니다.
- `showLiveStatus` - 댓글 위에 채팅 스타일의 "Live" + 사용자 수 헤더 스트립을 표시합니다.
- `useInlineSubmitButton` - 컴포저 내부에 아이콘 형태의 전송 버튼을 렌더링합니다.
- `countAboveToggle` - `useShowCommentsToggle`와 함께 사용할 때, "Show Comments" 토글 위에 몇 개의 댓글을 렌더링할지 지정합니다.
- `preserveFeedScrollPosition` - `FastCommentsFeed`는 언마운트/리마운트 간에 스크롤 오프셋을 기억합니다 (기본값 true).

### FastComments Concepts

The main concepts to be aware of to get started are `tenantId` and `urlId`. `tenantId` is your FastComments.com account identifier. `urlId` is where comment threads
will be tied to. This could be a page URL, or a product id, an article id, etc.

### Localization

All user-facing text in these widgets (button labels, placeholders, empty states, relative
dates like "5 minutes ago", error messages, etc.) is **server-driven**. The components do not
hard-code English strings; they render the translations FastComments serves for the requested
locale.

To request a locale, set `locale` in your config:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

When no `locale` is set, FastComments serves the tenant's default language.

**Editing the text:** translations are managed in your FastComments dashboard, not in this SDK.
To change wording, override the default copy, or add a language, edit the translations for your
account in the dashboard - the change is picked up by the widgets automatically with no app
release required. The SDK ships no English fallbacks, so any key you blank out in the dashboard
renders empty; keep the keys populated for every locale you support.

### User Notifications

FastComments supports notifications for [many scenarios](https://docs.fastcomments.com/guide-notifications.html). Notifications are configurable,
can be opted-out globally or at a notification/comment level, and supports page-level subscriptions so that users can subscribe to threads of a
specific page or article.

For example, it is possible to use Secure SSO to authenticate the user and then periodically poll for unread notifications and push them to the user.

See [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for how to get and translate unread user notifications.

### Gif Browser

By default, no image or gif selection is enabled. See [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for how
to support image and gif uploads. There is a Gif Browser that anonymizes searches and images provided in this library, you simply have to use it.

### Performance

Please open a ticket with an example to reproduce, including device used, if you identify any performance problems. Performance is a first-class citizen
of all FastComments libraries.