---
在 FastComments，我们使用相同的 API 编写自己的扩展。您可以在以下端点查看这些扩展的未压缩代码：

#### 深色模式

深色模式扩展在检测到 "dark" 页面时有条件地加载。该扩展只是向评论小部件添加一些 CSS。这样当不需要时我们就不必加载深色模式的 CSS。

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### 审核

当当前用户是版主时，我们会使用审核扩展。

这是一个用于添加基于点击的事件监听器、发起 API 请求、向评论菜单和评论区域添加项的好示例。

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### 实时聊天

实时聊天扩展（结合其他配置和样式）可以将评论小部件变为实时聊天组件。

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js

---