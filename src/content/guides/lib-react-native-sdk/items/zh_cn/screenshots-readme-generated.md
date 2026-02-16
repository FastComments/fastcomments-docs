---
#### 皮肤：Erebus
![皮肤：Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### 皮肤：默认
![皮肤：默认](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### 原生所见即所得(WYSIWYG)编辑器，支持图片！
![原生所见即所得(WYSIWYG)编辑器，支持图片](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### 富文本编辑器

本库使用 10tap 编辑器提供富文本编辑功能，带来强大的所见即所得编辑体验。

### 配置选项

本库旨在支持 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定义的所有配置选项，类似于 Web 实现。

### FastComments 概念

要开始使用，需要了解的主要概念是 `tenantId` 和 `urlId`。`tenantId` 是你在 FastComments.com 的账户标识符。`urlId` 用于将评论线程关联到某处。这可以是页面 URL、产品 ID、文章 ID 等。

### 用户通知

FastComments 支持[许多场景](https://docs.fastcomments.com/guide-notifications.html)的通知。通知是可配置的，可以在全局或单个通知/评论级别选择退出，并且支持页面级订阅，用户可以订阅特定页面或文章的线程。

例如，可以使用 Secure SSO 对用户进行身份验证，然后定期轮询未读通知并将其推送给用户。

参见 [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) 了解如何获取并翻译未读用户通知。

### Gif 浏览器

默认情况下，不启用图片或 gif 选择。参见 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支持图片和 gif 上传。本库提供了一个 Gif 浏览器，会对搜索和提供的图片进行匿名化，你只需使用它即可。

### 性能

如果你发现任何性能问题，请提交一个包含可重现示例（包括所用设备）的工单。性能是所有 FastComments 库的首要关注点。
---