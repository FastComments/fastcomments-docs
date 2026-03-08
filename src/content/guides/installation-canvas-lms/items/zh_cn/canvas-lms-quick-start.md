1. 登录 FastComments 并前往 <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">我的账户 > Canvas LTI 配置</a>。
2. 输入 **配置名称** 和您的 **平台 URL**（例如 `https://yourschool.instructure.com`），然后单击 **创建配置**。向导将进入第 2 步并显示您的 **配置 URL**。
3. 在 Canvas 中，前往 **管理员 > 开发者密钥 > + 开发者密钥 > LTI 密钥**。将 **Method** 设置为 "Enter URL" 并粘贴配置 URL。保存密钥并将其 **State** 设置为 **ON**。
4. 从 Canvas 的开发者密钥表中复制 **Client ID** 编号。回到 FastComments，将其粘贴到 **Client ID** 字段并单击 **保存并继续**。
5. 查看配置摘要，然后单击 **启用集成** 以上线。
6. 在您的 Canvas 课程中，前往 **设置 > 导航**，找到 **FastComments** 并启用它。评论将显示为课程导航项。