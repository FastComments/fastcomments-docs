Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>实时评论</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="实时评论，浅色主题"/></td>
    <td align="center"><b>暗色主题</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="实时评论，暗色主题"/></td>
    <td align="center"><b>实时聊天</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="实时聊天预设"/></td>
  </tr>
</table>

### 富文本编辑器

此库使用 [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) 进行富文本编辑，提供强大的所见即所得编辑体验。同一编辑器支持 iOS、Android 和网页（通过 `react-native-web`），因此编辑器在所有平台上表现一致，采用单一实现。

`react-native-enriched` 需要在原生端使用 React Native 新架构（Fabric）（自 RN 0.76 起默认启用，RN 0.72-0.75 需要手动开启），并且需要能够解析 package `exports` 条件的打包工具。此 SDK 开发并测试于 RN 0.81 / React 19。相同的编辑器也可通过 `react-native-web` 在网页端运行；enriched 编辑器的网页构建仍被标记为上游实验性。

### 小部件

SDK 提供了三个小部件，映射自 FastComments Android SDK：

- `FastCommentsLiveCommenting` - 支持投票、回复、分页、提及、通知以及实时更新的线程评论。
- `FastCommentsLiveChat` - 基于相同引擎的聊天预设：消息按时间顺序排列，新消息在底部，编辑器位于列表下方，顶部有实时状态条（连接点 + 用户计数），通过向上滚动加载无限历史记录，自动滚动到新消息，不支持投票或回复线程。所有预设均可通过 `config` 覆盖。
- `FastCommentsFeed` - 带有发布编辑器、媒体、互动、关注以及实时新帖横幅的社交信息流。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### 主题

默认外观由一套语义化设计令牌（`FastCommentsTheme`）生成：颜色、间距、圆角、字体大小、字体粗细和头像尺寸。通过任意小部件的 `theme` 属性传入部分令牌覆盖（类型为 `FastCommentsThemeOverrides`），即可一致地重新样式化整个样式树：

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

暗色模式只需切换一套令牌：

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` 属性仍然接受原始的 `IFastCommentsStyles` 树，以实现精细控制。当同时提供 `theme` 和 `styles` 时，显式的 `styles` 会覆盖主题树；当仅提供 `styles` 时，它会完全替代默认样式（保持原有行为，现有集成和皮肤不受影响）。`setupDarkModeSkin` 已被弃用，建议使用 `theme` 属性。

### 配置选项

此库旨在支持在 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定义的所有配置选项，类似于网页实现。

在此基础上，React Native 通过 `FastCommentsRNConfig` 添加了一些 SDK 特有的选项：

- `hideTopBar` - 隐藏显示在编辑器上方的已登录用户/通知铃铛条。
- `usePressToEdit` - 长按评论以打开其菜单。
- `disableDownVoting` - 隐藏点踩按钮。
- `renderCommentInline` - 在与评论内容相同的 HTML 块内渲染评论者信息。
- `renderLikesToRight` - 将投票/点赞区域移动到评论右侧，而非下方。
- `renderDateBelowComment` - 在评论下方渲染日期。
- `showLiveStatus` - 在评论上方显示聊天式的 “Live” + 用户计数 状态条。
- `useInlineSubmitButton` - 将提交按钮渲染为编辑器内部的图标按钮。
- `countAboveToggle` - 与 `useShowCommentsToggle` 配合使用，指定在 “显示评论” 切换按钮上方渲染的评论数量。
- `preserveFeedScrollPosition` - `FastCommentsFeed` 在卸载/重新挂载之间记住滚动偏移（默认 true）。

### FastComments 概念

入门时需要了解的主要概念是 `tenantId` 和 `urlId`。`tenantId` 是您在 FastComments.com 的账户标识。`urlId` 用于绑定评论线程的位置，可以是页面 URL、产品 ID、文章 ID 等。

### 本地化

这些小部件中所有面向用户的文本（按钮标签、占位符、空状态、类似 “5 分钟前” 的相对日期、错误信息等）均为 **服务器驱动**。组件不会硬编码英文字符串，而是渲染 FastComments 为请求的语言提供的翻译。

要请求特定语言，请在配置中设置 `locale`：

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

如果未设置 `locale`，FastComments 将使用租户的默认语言。

**编辑文本：** 翻译在您的 FastComments 仪表盘中管理，而非此 SDK。若要更改措辞，可覆盖默认文案或添加语言，在仪表盘中编辑您账户的翻译——小部件会自动获取更改，无需发布应用。SDK 不提供英文回退，因此在仪表盘中留空的键会渲染为空；请为您支持的每种语言填充键值。

### 用户通知

FastComments 支持针对 [多种场景](https://docs.fastcomments.com/guide-notifications.html) 的通知。通知可配置，可在全局或单个通知/评论层面选择退出，并支持页面级订阅，用户可以订阅特定页面或文章的线程。

例如，可以使用 Secure SSO 对用户进行身份验证，然后定期轮询未读通知并推送给用户。

请参阅 [示例 AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) 了解如何获取并翻译未读用户通知。

### Gif 浏览器

默认情况下，未启用图片或 gif 选择。请参阅 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支持图片和 gif 上传。该库提供了一个 Gif 浏览器，可对搜索和提供的图片进行匿名化，您只需使用它。

### 性能

如果发现任何性能问题，请提交包含复现示例和使用设备信息的工单。性能是所有 FastComments 库的首要关注点。