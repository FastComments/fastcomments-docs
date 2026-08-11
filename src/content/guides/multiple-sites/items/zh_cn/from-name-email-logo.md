有时 FastComments 需要给用户发送电子邮件，尤其是在您未使用安全单点登录（Secure SSO）时。

这类情况包括在用户首次评论时验证其账户或活动。FastComments 还会向他们发送评论回复的通知。

当 FastComments 给您的用户发送电子邮件时，我们会使用默认的发件人名称和电子邮件地址 `FastComments Robot` 与 `noreply@fastcomments.com`。

我们还会在这些电子邮件的页脚使用我们的徽标。

如果您使用的是 FastComments Flex 或 Pro，所有这些都可以通过 “My Domains” 页面按域进行自定义：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='每个域的电子邮件设置表单，包含发件人名称、发件人邮箱和徽标上传字段'; title='自定义发件人名称、邮箱和徽标' app-screenshot-end]

在自定义电子邮件中显示的徽标时，请确保您上传的尺寸与希望在电子邮件页脚中显示的尺寸相同。

### 自定义 `From Domain` 时

如果您自定义了 `From Domain`，电子邮件提供商和客户端需要知道 FastComments 已获授权代表您发送电子邮件。否则，仅定义 `From Domain` 而不遵循以下步骤，可能会导致电子邮件进入垃圾邮件文件夹。

#### 1. 设置 SPF

为了让 FastComments 能安全地以您的域名发送电子邮件，请确保添加允许我们发送的 SPF 记录。

确保有 SPF 记录允许 `mail.fastcomments.com` 和 `sib.fastcomments.com` 以您的域名发送邮件。

有关如何操作的更多信息，请参阅：https://mailtrap.io/blog/multiple-spf-records/

#### 2. 设置 DKIM

除了 SPF，您还应设置 DKIM。DNS 配置完成后，您可以在域名配置页面点击 “Show Advanced” 来显示每个域的 DKIM 设置。

您也可以 [invoke the API](/guide-api.html#domain-config-structure) 来设置 DKIM 配置。

### 退订链接

使用 SSO 时，电子邮件和通知中的退订功能可以通过 [DomainConfigs API](/guide-api.html#domain-config-structure) 进行自定义。

### 电子邮件链接混淆

如果您站点的域名声誉导致通知邮件进入垃圾邮件，您可以将 “查看评论” 按钮的链接改为通过 `fastcomments.com` 路由，而不是直接链接到您的页面。邮箱提供商会根据链接目标的声誉对电子邮件正文中的每个链接进行评分，因此当您的域名被标记时，裸链接会提升垃圾邮件分数，无论您的发送设置多么干净。

在 “My Domains” 页面下的 “Show Advanced” 中的 “Email Link Obfuscation” 部分启用此功能。此设置按域生效。

启用后，提及、回复、新评论、已订阅页面、个人资料评论和摘要邮件中的链接会被重写为短令牌，点击后重定向到原始页面。目标绑定到您的租户：重定向仅转发到主机匹配您配置域名的 URL，令牌在 30 天后自动过期。

点击后的体验保持不变。读者仍会在您的页面上看到已滚动到视图中的评论。