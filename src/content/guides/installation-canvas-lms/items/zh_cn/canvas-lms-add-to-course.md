#### 评论如何在您的课程中显示

一旦启用了 LTI 集成并安装了外部应用（External App），FastComments 会根据您配置的放置位置自动运行：

#### 作业视图（Assignment View）

如果启用了 **Assignment View** 放置位置，评论会自动显示在课程中每个作业的下方。学生和教师在查看作业时会看到一个分层的评论区——无需对每个作业进行额外设置。

每个作业都有自己独立的评论线程。

#### 富文本编辑器按钮（Rich Content Editor Button）

如果启用了 **Editor Button** 放置位置，教师可以将 FastComments 嵌入到任何使用富文本编辑器的内容中：

1. 编辑一个 **Page**（页面）、**Quiz**（测验）或 **Announcement**（公告）。
2. 在富文本编辑器工具栏中，点击 **FastComments** 按钮。
3. FastComments 会自动嵌入到内容中。
4. 保存页面。

当学生查看该页面时，嵌入的 FastComments 组件会加载该页面独有的评论线程。

#### 自动 SSO

在这两种放置位置中，学生都会通过其 Canvas 账户自动登录。姓名、电子邮件和头像通过 LTI 启动进行同步，无需单独登录。

#### 锁定公开访问（推荐）

默认情况下，FastComments 的评论数据是公开可读的。任何能够猜到线程 URL 或 API 端点的人都可以查看其评论，即使在 Canvas 之外也是如此。对于课程讨论，您几乎肯定希望将查看权限限制为仅限已注册的学生。

打开您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">组件自定义页面</a>，创建一条启用 **Require SSO To View Comments** 的规则，然后将安全级别设置为 **Secure SSO**，这样评论线程只能通过经过签名的 LTI 启动加载。

完整操作步骤请参阅 [使用单点登录保护评论线程](/guide-customizations-and-configuration.html#sso-require-to-view-comments)，包括如何将规则限定到单个域名或页面。