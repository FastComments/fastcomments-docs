#### 如何在您的课程中显示评论

一旦启用 LTI 集成并安装了 External App，FastComments 会根据您配置的放置点自动工作：

#### Assignment View

如果启用了 **Assignment View** 放置点，评论会自动显示在课程中每个作业的下方。学生和教师在查看作业时会看到一个线程式评论区 — 每个作业不需要额外设置。

每个作业都有自己独立的评论线程。

#### Rich Content Editor Button

如果启用了 **Editor Button** 放置点，教师可以在使用 Rich Content Editor 的任何内容中嵌入 FastComments：

1. 编辑一个 **Page**、**Quiz** 或 **Announcement**。
2. 在 Rich Content Editor 工具栏中，点击 **FastComments** 按钮。
3. FastComments 会自动嵌入到内容中。
4. 保存页面。

当学生查看该页面时，嵌入的 FastComments 小部件会加载该页面专属的评论线程。

#### Automatic SSO

在这两种放置点中，学生会通过他们的 Canvas 帐户自动登录。姓名、电子邮件和头像通过 LTI 启动同步，无需单独登录。

#### Lock Down Public Access (Recommended)

默认情况下，FastComments 的评论数据是公开可读的。任何能够猜到线程 URL 或 API endpoint 的人都可以查看其评论，甚至在 Canvas 之外也是如此。对于课程讨论，您几乎肯定希望将查看权限限制为仅限已注册的学生。

打开您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a>，并创建一条启用 **Require SSO To View Comments** 的规则，然后将安全级别设置为 **Secure SSO**，这样线程只能通过签名的 LTI 启动加载。

有关完整的操作指南（包括如何将规则限定到单个域或页面），请参阅 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments)。