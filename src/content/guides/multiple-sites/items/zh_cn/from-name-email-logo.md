有时 FastComments 需要给您的用户发送电子邮件，尤其是在您未使用 Secure SSO 时。

例如，包括在用户首次评论时验证他们的帐户或活动。

FastComments 还会发送回复其评论的通知。

当 FastComments 给您的用户发送电子邮件时，我们会使用默认的发件人名称和电子邮件：`FastComments Robot` 和 `noreply@fastcomments.com`。

我们还会在这些邮件的页脚使用我们自己的徽标。

如果您有 FastComments Flex 或 Pro，您可以通过“我的域”页面按域对这些内容进行自定义：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

在自定义电子邮件中显示的徽标时，请确保您上传的尺寸与希望在邮件页脚中显示的尺寸相同。

### 自定义 `From Domain` 时

如果您自定义了 `From Domain`，电子邮件提供商和客户端需要知道 FastComments 被授权代表您发送电子邮件。否则，仅定义 `From Domain` 而不遵照下面的步骤，很可能会导致邮件被归入垃圾邮件。

#### 1. 设置 SPF

为了允许 FastComments 安全地以您的域名发送电子邮件，请确保添加一个允许我们这样做的 SPF 记录。

请确保存在 SPF 记录，允许 `mail.fastcomments.com` 和 `sib.fastcomments.com` 以您的域名发送邮件。

有关如何执行此操作的更多信息，请参见： https://mailtrap.io/blog/multiple-spf-records/

#### 2. 设置 DKIM

除了 SPF，您还应该设置 DKIM。DNS 配置就绪后，您可以在域配置页面点击“显示高级”来显示每个域的 DKIM 设置。

您也可以[调用 API](/guide-api.html#domain-config-structure) 来设置 DKIM 配置。

### 退订链接

在使用 SSO 时，电子邮件和通知中使用的退订功能可以通过 [DomainConfigs API](/guide-api.html#domain-config-structure) 进行自定义。

### 电子邮件链接混淆

如果您网站的域名信誉导致通知邮件被判为垃圾邮件，您可以将“查看评论”按钮通过 `fastcomments.com` 进行路由，而不是直接链接到您的页面。邮箱提供商会根据目标的信誉为邮件正文中的每个链接打分，因此当您的域名被标记时，即使您的发送设置很干净，裸链接也会提高垃圾邮件评分。

在“我的域”页面的“显示高级”中，在“邮件链接混淆”部分启用此项。该设置为每域单独配置。

启用后，mention、reply、new-comment、subscribed-page、profile-comment 和 digest 邮件中的链接会被重写为短令牌，点击后重定向到原始页面。目标与您的租户绑定：重定向仅转发到主机与您配置的某一域匹配的 URL，且令牌会在 30 天后自动失效。

点击后的体验保持不变。读者仍然会到达您的页面，并自动滚动到相应的评论位置。

---