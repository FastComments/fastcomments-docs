FastComments 通过 SSO（单点登录）与 Drupal 的用户系统集成。您的用户在 Drupal 站点登录后，模块会自动将他们的身份传递给 FastComments。无需创建额外账户，也不需要执行初始同步。

该模块支持三种 SSO 模式，可在 `Administration > Configuration > Content > FastComments` 下设置。

### 无

无 SSO。用户以访客身份发表评论或创建 FastComments 账号。如果您的站点是公开的且无需将评论绑定到 Drupal 用户，请使用此模式。

### 简单

将 Drupal 用户的姓名、电子邮件和头像传递给 FastComments，无需服务器端验证。不需要 API Secret。适合内部或低风险站点。

### 安全（推荐）

使用 [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) 向 FastComments 验证每个用户身份。当您配置了 API Secret 时，应使用此模式，它也是唯一能防止访问者冒充其他用户的模式。

每次用户查看评论线程时，用户身份都会传递给 FastComments。无需运行初始或持续的同步。

<sup>(可选)</sup> 将您的管理员添加到 [Users & Administrators](https://fastcomments.com/auth/my-account/users) ，将版主添加到 [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators) ，以改善他们的体验并为版主启用统计跟踪。

有关 SSO 工作原理的更深入说明，请参阅自定义文档的 [SSO 部分](/guide-customizations-and-configuration.html#sso)。

---