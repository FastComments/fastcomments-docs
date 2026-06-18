Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich Text Editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widgets

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` - threaded commenting with votes, replies, pagination, mentions, notifications, and live updates.
- `FastCommentsLiveChat` - a chat preset over the same engine: chronological messages with new ones at the bottom, the composer below the list, a live header strip (connection dot + user count), infinite history loaded by scrolling up, auto-scroll to new messages, no votes or reply threading. Every preset can be overridden via `config`.
- `FastCommentsFeed` - a social feed with post composer, media, reactions, follows, and live new-post banners.

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

- `hideTopBar` - hide the logged-in user / notification-bell strip shown above the composer.
- `usePressToEdit` - press-and-hold a comment to open its menu.
- `disableDownVoting` - hide down-vote buttons.
- `renderCommentInline` - render commenter info inside the same HTML block as the comment content.
- `renderLikesToRight` - move the vote/like area to the right of the comment instead of below it.
- `renderDateBelowComment` - render the date below the comment.
- `showLiveStatus` - show the chat-style "Live" + user-count header strip above comments.
- `useInlineSubmitButton` - render the submit button as an icon inside the composer.
- `countAboveToggle` - with `useShowCommentsToggle`, how many comments render above the "Show Comments" toggle.
- `preserveFeedScrollPosition` - `FastCommentsFeed` remembers its scroll offset across unmount/remount (default true).

### FastComments Concepts

The main concepts to be aware of to get started are `tenantId` and `urlId`. `tenantId` is your FastComments.com account identifier. `urlId` is where comment threads
will be tied to. This could be a page URL, or a product id, an article id, etc.

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