Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>实时评论</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>深色主题</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>实时聊天</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich Text Editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widgets

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` - 带有投票、回复、分页、提及、通知和实时更新的线程式评论。
- `FastCommentsLiveChat` - 基于相同引擎的聊天预设：按时间顺序排列的消息，新消息显示在底部，撰写器位于列表下方，具有实时头部条（连接点 + 用户计数），通过向上滚动加载无限历史，自动滚动到新消息，不包含投票或回复线程。每个预设都可以通过 `config` 覆盖。
- `FastCommentsFeed` - 带帖子撰写器、媒体、反应、关注和实时新帖横幅的社交信息流。

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

- `hideTopBar` - 隐藏显示在撰写器上方的已登录用户 / 通知铃声条。
- `usePressToEdit` - 长按评论以打开其菜单。
- `disableDownVoting` - 隐藏点踩按钮。
- `renderCommentInline` - 在与评论内容相同的 HTML 块内渲染评论者信息。
- `renderLikesToRight` - 将投票/点赞区域移到评论的右侧，而不是下方。
- `renderDateBelowComment` - 在评论下方渲染日期。
- `showLiveStatus` - 在评论上方显示聊天风格的“Live”+用户计数头部条。
- `useInlineSubmitButton` - 将提交按钮作为撰写器内的图标渲染。
- `countAboveToggle` - 与 `useShowCommentsToggle` 配合使用时，在“显示评论”切换上方渲染多少条评论。
- `preserveFeedScrollPosition` - `FastCommentsFeed` 在卸载/重新挂载之间记住其滚动偏移（默认 true）。

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