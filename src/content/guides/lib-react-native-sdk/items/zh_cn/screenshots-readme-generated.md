实时线程式评论，支持头像、嵌套回复、投票和内置富文本编辑器，此外还提供深色主题和实时聊天预设（此处展示的是通过 `react-native-web` 渲染的效果）：

<table>
  <tr>
    <td align="center"><b>实时评论</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="实时评论，浅色主题"/></td>
    <td align="center"><b>深色主题</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="实时评论，深色主题"/></td>
    <td align="center"><b>实时聊天</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="实时聊天预设"/></td>
  </tr>
</table>

### 富文本编辑器

本库使用 [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) 进行富文本编辑，提供强大的所见即所得编辑体验。同一编辑器同时支持 iOS、Android 和 Web（通过 `react-native-web`），因此只需一套实现，编辑器在所有平台上的表现都保持一致。

`react-native-enriched` 在原生端需要 React Native 新架构（Fabric）（自 RN 0.76 起为默认，RN 0.72-0.75 需手动启用），并且需要能够解析包 `exports` 条件的打包器。本 SDK 基于 RN 0.81 / React 19 开发和测试。同一编辑器也可通过 `react-native-web` 在 Web 上运行；富文本编辑器的 Web 构建在上游仍被标记为实验性。

### 组件

SDK 提供三个组件，与 FastComments Android SDK 保持一致：

- `FastCommentsLiveCommenting` - 支持投票、回复、分页、提及、通知和实时更新的线程式评论。
- `FastCommentsLiveChat` - 基于同一引擎的聊天预设：按时间顺序排列消息，新消息显示在底部，编辑器位于列表下方，顶部有实时状态栏（连接指示点 + 用户数），向上滚动加载无限历史记录，自动滚动到新消息，无投票和回复线程。每个预设都可以通过 `config` 覆盖。
- `FastCommentsFeed` - 社交动态，包含发帖编辑器、媒体、表情反应、关注和实时新帖横幅。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### 主题

默认外观由一组语义化设计令牌（`FastCommentsTheme`）生成：颜色、间距、圆角、字号、字重和头像尺寸。通过任意组件的 `theme` 属性传入部分令牌覆盖（类型为 `FastCommentsThemeOverrides`），整个样式树将一致地重新应用样式：

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

深色模式只需更换一套令牌：

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` 属性仍然接受原始的 `IFastCommentsStyles` 树，用于精细控制。当同时提供 `theme` 和 `styles` 时，显式样式优先于主题样式树；当仅提供 `styles` 时，它将完全替换默认样式（即原始行为，因此现有集成和皮肤不受影响）。`setupDarkModeSkin` 已弃用，建议使用 `theme` 属性。

### 配置选项

本库旨在支持 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定义的所有配置选项，与 Web 实现一致。

除此之外，React Native 通过 `FastCommentsRNConfig` 增加了一些 SDK 特有的选项：

- `hideTopBar` - 隐藏显示在编辑器上方的已登录用户/通知铃铛栏。
- `usePressToEdit` - 长按评论以打开其菜单。
- `disableDownVoting` - 隐藏踩（反对票）按钮。
- `renderCommentInline` - 将评论者信息渲染在与评论内容相同的 HTML 块中。
- `renderLikesToRight` - 将投票/点赞区域移至评论右侧而非下方。
- `renderDateBelowComment` - 将日期渲染在评论下方。
- `showLiveStatus` - 在评论上方显示聊天风格的"实时" + 用户数状态栏。
- `useInlineSubmitButton` - 将提交按钮渲染为编辑器内的图标。
- `countAboveToggle` - 配合 `useShowCommentsToggle` 使用，指定在"显示评论"开关上方渲染多少条评论。
- `preserveFeedScrollPosition` - `FastCommentsFeed` 在卸载/重新挂载之间记住其滚动位置（默认为 true）。

### FastComments 核心概念

入门时需要了解的主要概念是 `tenantId` 和 `urlId`。`tenantId` 是您的 FastComments.com 账户标识符。`urlId` 是评论线程所绑定的对象，可以是页面 URL、产品 ID、文章 ID 等。

### 本地化

这些组件中所有面向用户的文本（按钮标签、占位符、空状态、诸如"5 分钟前"之类的相对日期、错误消息等）都是**由服务器驱动的**。组件不会硬编码英文字符串；它们渲染 FastComments 为所请求的语言环境提供的翻译。

要请求某个语言环境，请在配置中设置 `locale`：

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es 等。
};
```

未设置 `locale` 时，FastComments 会提供租户的默认语言。

**编辑文本：** 翻译在您的 FastComments 控制面板中管理，而不是在本 SDK 中。要更改措辞、覆盖默认文案或添加语言，请在控制面板中编辑您账户的翻译——更改会被组件自动获取，无需发布应用。SDK 不内置英文回退文本，因此您在控制面板中清空的任何键都会渲染为空；请为您支持的每个语言环境保持这些键有内容。

### 用户通知

FastComments 支持[多种场景](https://docs.fastcomments.com/guide-notifications.html)的通知。通知是可配置的，可以在全局或单个通知/评论级别选择退出，并支持页面级订阅，使用户可以订阅特定页面或文章的评论线程。

例如，可以使用安全 SSO 对用户进行身份验证，然后定期轮询未读通知并将其推送给用户。

请参阅 [AppNotificationSecureSSO 示例](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)，了解如何获取和翻译用户未读通知。

### Gif 浏览器

默认情况下，未启用图片或 Gif 选择功能。请参阅 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支持图片和 Gif 上传。本库提供了一个 Gif 浏览器，可对搜索和图片进行匿名化处理，您只需使用它即可。

### 性能

如果您发现任何性能问题，请提交工单并附上可复现的示例，包括所使用的设备。性能是所有 FastComments 库的一等公民。