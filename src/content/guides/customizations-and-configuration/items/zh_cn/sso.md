SSO，或单点登录，是一组约定，用于允许您或您的用户在不必创建另一个账号的情况下使用 FastComments。

假设您不允许匿名评论，发表评论需要一个账号。我们让这个注册流程非常简单——用户在评论时只需留下他们的邮箱。
不过，我们也理解即便如此，某些站点仍希望尽量减少摩擦。

我们可以通过为整个站点只保留一个登录流程来减少这种摩擦。

### How do I get it?
所有账号类型目前都可访问 SSO。不过，SSO 用户的最大数量会根据您的套餐而有所不同。与其他功能一样，Pro 计划及以上提供直接的开发支持。

我们先比较这些选项，然后详细介绍每一种。

### User and Comment Migrations

从像 Disqus 这样的具有 SSO 的平台迁移时，您通常已经拥有用户及其评论。

评论作为迁移的一部分被导入，可以通过 API、我们的导入 UI 或客服支持完成。如果导入 UI 支持您正在迁移的平台，则优先使用导入 UI，因为它集成了错误处理、头像和媒体的提取与上传，以及批处理作业监控系统。

如果使用导入 UI（位于 `Manage Data -> Imports`）则头像及其他媒体会自动迁移。

用户本身会在首次查看评论线程时自动被添加。或者，也可以通过 API 预先添加，但这样做并没有太多优势。

如果评论被导入，而 SSO 用户没有通过 API 手动添加，那么当用户首次查看任何评论线程并创建账户时，评论会自动迁移到该用户的账户。之后他们就可以管理、编辑和删除他们最初发表的评论。

自动迁移是通过邮箱或用户名完成的。有些平台在导出时不提供邮箱，例如 Disqus，因此在这种情况下我们会回退到使用用户名。
- 只要您在 SSO payload 中传入匹配的用户名和一个邮箱，我们就会将该邮箱添加到各个评论对象中，以便通知和提及功能能够正常工作。

如果希望在一次性导入评论和用户，可以在通过 API 导入用户后与支持团队合作，将评论迁移到各自用户的账户。

总结来说，迁移的最简路径是：

1. 导入评论。
   1. 如果使用 `Manage Data -> Imports` 中的导入 UI，头像和其他媒体会自动迁移。
2. 设置 Secure 或 Simple SSO。
3. 让迁移在每个用户首次登录时自动发生。
   1. 如果用户的评论少于 50k，通常这会在页面加载时增加不到一秒的时间。

### WordPress Users
如果您使用我们的 <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress 插件</a>，则无需编写任何代码！只需转到插件的管理页面，点击 SSO 设置，然后启用。

这会带您进入一个单按钮向导，该向导将为您创建 API key，将其发送到您的 WordPress 安装并开启 SSO。我们已将此过程整合为单按钮操作以简化流程。

注意，如果您是第一次安装该插件，则必须完成设置流程，之后才能看到包含 SSO 设置按钮的管理页面。

#### WordPress SSO - Moderators

请注意，当前要在使用 FastComments WordPress 插件评论时在版主旁显示 “Moderator” 徽章，
他们还必须在 FastComments 仪表板中被添加为版主，并且其邮箱已通过验证。

### Custom Integrations

对于自定义集成，有两种选择。

### Option One - Secure SSO

使用 Secure SSO 时，FastComments 可以确定正在发表评论、投票和阅读评论的用户确实是您站点上的真实用户。

只要您创建了有效的 payload，用户就会始终拥有无缝的评论体验。

使用 Secure SSO 时，SSO payload 在 **服务器端** 使用 HMAC 验证创建，然后传递到 **客户端** 的小部件中。

使用 Secure SSO 时，用户的账户与其余 FastComments 用户库是 **完全独立** 的。这意味着如果我们有两个合作方 Company A 和 Company B，每一方都可以有一个用户名为 "Bob" 的 SSO 用户。

#### Requirements
- 需要一些后端开发基础知识。
- 需要了解如何处理秘密 API 密钥。
- 需要一些 API 开发或服务器端渲染的基础知识。

#### Pros
- 安全。
- 无缝的评论体验。

#### Cons
- 需要后端开发。

#### Updating User Data

使用 Secure SSO 时，每次您传递 SSO 用户 payload，我们都会用最新信息更新该用户。例如，如果
用户的用户名为 `X`，而您在 SSO payload 中传入 `Y`，那么他们的用户名将变为 `Y`。

如果您想使用这种方法移除某些值，请将其设置为 `null`（而非 `undefined`）。

#### Secure SSO API

我们还提供与 SSO 用户交互的 API。参见 [文档](/guide-api.html#sso-user-structure)。

注意，在使用 Secure SSO 时，用户会在页面加载时在后台自动创建。您不必批量导入用户。

### Option Two - Simple SSO

Secure SSO 的替代方案是简单地将用户信息传递给评论小部件。

使用 Simple SSO 时并不强制提供邮箱，但如果没有邮箱，他们的评论将显示为 “Unverified”。

<sup>注意！</sup> 截至 2022 年初，使用 Simple SSO 的用户名不需要在整个 FastComments.com 上唯一。

理想情况下，仅当在不提供后端访问的平台上开发时才选择 Simple SSO。

#### Requirements
- 需要一些客户端开发的基础知识。
- 至少需要知道用户的邮箱。

#### Pros
- 简单。
- 所有活动仍会被验证。
- 用户无需输入其用户名或邮箱。

#### Cons
- 比 Secure SSO 安全性低，因为客户端的 payload 可能被伪造以冒充任意用户。

#### Simple SSO API

通过 Simple SSO 流程自动创建的用户以 `SSOUser` 对象形式存储。可以通过 `SSOUser` API 访问和管理它们。参见 [文档](/guide-api.html#sso-user-structure)。