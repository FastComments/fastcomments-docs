1. 转到 <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">您的 FastComments LTI 配置</a>。
2. 输入一个 **配置名称** 和您的 **平台 URL**（例如 `https://yourschool.instructure.com`）。选择要启用的 **放置位置**（作业查看和/或编辑器按钮 — 两者默认均已开启）。点击 **创建配置**。向导将进入第 2 步并显示您的 **配置 URL**。
3. 在 Canvas 中，转到 **Admin > Developer Keys > + Developer Key > LTI Key**。将 **Method** 设置为 “输入 URL”，并粘贴配置 URL。保存密钥，然后将其 **State** 设置为 **ON**，并在提示时点击 **允许**。
4. 从 Canvas 的 Developer Keys 表中复制 **Client ID** 号码。在 FastComments 中，将其粘贴到 **Client ID** 字段并点击 **保存并继续**。
5. 查看配置摘要并点击 **启用集成** 以使其上线。
6. 在 Canvas 中安装该外部应用（**Admin > Settings > Apps > + App > By Client ID**）。评论将自动出现在作业下方，教师可以通过富文本编辑器工具栏按钮将 FastComments 嵌入页面、测验和公告中。