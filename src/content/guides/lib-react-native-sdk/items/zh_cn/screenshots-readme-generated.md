Live threaded commenting with avatars, nested replies, votes, and the built-in rich‑text composer, plus a dark theme and a live‑chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>实时评论</b><br/><img src="./demo-screenshots/light.png" width="260" alt="实时评论，亮色主题"/></td>
    <td align="center"><b>暗色主题</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="实时评论，暗色主题"/></td>
    <td align="center"><b>实时聊天</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="实时聊天预设"/></td>
  </tr>
</table>

### 富文本编辑器

此库使用 [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) 提供富文本编辑功能，带来强大的所见即所得编辑体验。相同的编辑器驱动 iOS、Android 与 Web（通过 `react-native-web`），因此在每个平台上都能使用单一实现保持一致的编辑器行为。

`react-native-enriched` 需要在原生端使用 React Native 新架构（Fabric）（自 RN 0.76 起默认开启，RN 0.72‑0.75 可自行启用），并且需要一个能够解析 package `exports` 条件的打包工具。此 SDK 已在 RN 0.81 / React 19 上开发并测试。相同的编辑器也可通过 `react-native-web` 在 Web 端运行；该编辑器的 Web 构建仍被标记为上游实验性特性。

### 小部件

SDK 提供了三个小部件，映射自 FastComments Android SDK：

- `FastCommentsLiveCommenting` – 支持投票、回复、分页、提及、通知及实时更新的线程评论。
- `FastCommentsLiveChat` – 基于相同引擎的聊天预设：消息按时间顺序排列，最新消息在底部，编辑框位于列表下方，带有实时标题条（连接点 + 用户计数），通过向上滚动加载历史记录，自动滚动到新消息，不支持投票或回复线程。每个预设都可以通过 `config` 覆盖。
- `FastCommentsFeed` – 带有发帖编辑框、媒体、表情、关注和实时新帖横幅的社交动态。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### 主题化

默认外观由一组语义化设计令牌（`FastCommentsTheme`）生成：颜色、间距、圆角、字体大小、字体粗细和头像尺寸。通过任意小部件的 `theme` 属性传入部分令牌覆写（类型为 `FastCommentsThemeOverrides`），即可让整个样式树一致地重新渲染：

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

暗黑模式只需切换一套令牌：

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` 属性仍然接受原始的 `IFastCommentsStyles` 树以实现精细控制。当同时提供 `theme` 与 `styles` 时，显式的 `styles` 会覆盖主题树；当仅提供 `styles` 时，会完全替代默认样式（保持原有行为，现有集成和皮肤不受影响）。`setupDarkModeSkin` 已废弃，推荐使用 `theme` 属性。

### 配置选项

本库旨在支持在 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定义的所有配置选项，行为与 Web 实现保持一致。

在此基础上，React Native 通过 `FastCommentsRNConfig` 添加了若干 SDK 特有的选项：

- `hideTopBar` – 隐藏编辑框上方显示已登录用户和通知铃的条。
- `usePressToEdit` – 长按评论以打开其菜单。
- `disableDownVoting` – 隐藏踩投票按钮。
- `renderCommentInline` – 将评论者信息渲染在与评论内容同一 HTML 块内。
- `renderLikesToRight` – 将点赞/投票区域移至评论右侧，而非下方。
- `renderDateBelowComment` – 将日期渲染在评论下方。
- `showLiveStatus` – 在评论上方显示聊天式 “Live” + 用户计数的标题条。
- `useInlineSubmitButton` – 将提交按钮渲染为编辑框内的图标按钮。
- `countAboveToggle` – 与 `useShowCommentsToggle` 一起使用时，决定在 “显示评论” 切换上方渲染多少条评论。
- `preserveFeedScrollPosition` – `FastCommentsFeed` 在卸载/重新挂载之间记住滚动偏移（默认 true）。

### FastComments 概念

入门时需要了解的核心概念是 `tenantId` 与 `urlId`。`tenantId` 是您在 FastComments.com 的账户标识。`urlId` 则是评论线程绑定的对象，可以是页面 URL、商品 ID、文章 ID 等。

### 本地化

这些小部件中所有面向用户的文本（按钮标签、占位符、空状态、类似 “5 分钟前” 的相对日期、错误信息等）均为 **服务器驱动**。组件不会硬编码英文字符串，而是渲染 FastComments 为所请求语言提供的翻译。

要请求特定语言，请在配置中设置 `locale`：

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, 等
};
```

如果未设置 `locale`，FastComments 将使用租户的默认语言。

**编辑文本：** 翻译在您的 FastComments 仪表板中管理，而非此 SDK。若需更改文案，可覆盖默认拷贝或添加语言，在仪表板中编辑对应账户的翻译——更改会自动被小部件拾取，无需发布新版本。SDK 不提供英文回退，仪表板中留空的键会导致对应语言显示为空；请确保为所有支持的语言填充键值。

### 用户通知

FastComments 支持针对[多种场景](https://docs.fastcomments.com/guide-notifications.html)的通知。通知可配置，用户可以全局或按通知/评论层面选择退出，并且支持页面级别订阅，用户可订阅特定页面或文章的线程。

例如，可以使用安全单点登录（SSO）对用户进行身份验证，然后定期轮询未读通知并推送给用户。

请参阅 [AppNotificationSecureSSO 示例](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)了解如何获取并本地化未读用户通知。

### Gif 浏览器

默认情况下，未启用图片或 gif 选择。请参阅 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx)了解如何支持图片和 gif 上传。库中提供了一个 Gif 浏览器，可对搜索和图片进行匿名化处理，只需直接使用即可。

### 性能

如果发现任何性能问题，请提交包含复现示例的工单，并提供所使用的设备信息。性能是所有 FastComments 库的头等大事。