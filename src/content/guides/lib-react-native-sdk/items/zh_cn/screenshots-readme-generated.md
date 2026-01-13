---
#### 皮肤：Erebus
![皮肤：Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### 皮肤：Default
![皮肤：Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### 原生 WYSIWYG 编辑器，支持图片！
![原生 WYSIWYG 编辑器，支持图片](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### 富文本编辑器

此库使用 10tap 编辑器提供富文本编辑功能，带来强大的所见即所得编辑体验。

### 配置选项

此库旨在支持在 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定义的所有配置选项，与 Web 实现一致。

### FastComments 概念

入门需要了解的主要概念是 `tenantId` 和 `urlId`。`tenantId` 是您在 FastComments.com 的账户标识。`urlId` 则用于绑定评论线程。这可以是页面 URL、产品 id、文章 id 等。

### 用户通知

FastComments 支持 [许多场景](https://docs.fastcomments.com/guide-notifications.html) 的通知。通知可配置，
可以在全局或按通知/评论级别选择退出，并支持页面级订阅，用户可以订阅特定页面或文章的讨论线程。

例如，可以使用 Secure SSO 验证用户身份，然后定期轮询未读通知并将其推送给用户。

参见示例 [AppNotificationsSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) 了解如何获取并转换未读的用户通知。

### Gif 浏览器

默认情况下，不启用任何图片或 GIF 选择。参见 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支持图片和 GIF 上传。此库提供了一个 Gif 浏览器，会对搜索和所提供的图片进行匿名处理，您只需使用它即可。

### 性能

如果您发现任何性能问题，请提交工单并附上可复现的示例，包括所用设备。性能是所有 FastComments 库的首要关注点。
---