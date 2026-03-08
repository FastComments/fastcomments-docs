1. 登录 FastComments 并转到 <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">我的帐户 > Canvas LTI 配置</a>。
2. 输入 **配置名称** 和你的 **平台 URL**（例如 `https://yourschool.instructure.com`）。选择要启用的 **放置位置**（作业视图和/或编辑器按钮 —— 默认两者均已启用）。点击 **创建配置**。向导会进入第 2 步并显示你的 **配置 URL**。
3. 在 Canvas 中，转到 **管理员 > 开发者密钥 > + 开发者密钥 > LTI 密钥**。将 **方法** 设置为 “输入 URL” 并粘贴配置 URL。保存密钥，然后将其 **状态** 设置为 **ON**，并在提示时点击 **允许**。
4. 从 Canvas 的开发者密钥表中复制 **Client ID** 编号。回到 FastComments，将其粘贴到 **Client ID** 字段并点击 **保存并继续**。
5. 查看配置摘要并点击 **启用集成** 以上线。
6. 在 Canvas 中安装外部应用（**管理员 > 设置 > 应用 > + 应用 > 通过 Client ID**）。评论会自动显示在作业下方，教师可以通过富文本编辑器工具栏按钮在页面、测验和公告中嵌入 FastComments。