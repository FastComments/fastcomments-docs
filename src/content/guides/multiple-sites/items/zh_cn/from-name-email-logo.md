有时 FastComments 需要给你的用户发送电子邮件，特别是当你没有使用 Secure SSO 时。

例如，验证他们的账户或在首次发表评论时验证其活动。FastComments 还会向他们发送有关评论回复的通知。

当 FastComments 给你的用户发送电子邮件时，我们会使用默认的发件人名称和邮箱 `FastComments Robot` 和 `noreply@fastcomments.com`。

我们还会在这些邮件的页脚使用我们自己的徽标。

如果你拥有 FastComments Flex 或 Pro，可以在“我的域名”页面上按域名逐一自定义这些设置：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='自定义发件人名称、邮箱和徽标' app-screenshot-end]

在自定义邮件中显示的徽标时，请确保上传的尺寸就是你希望在邮件页脚中显示的尺寸。

### When Customizing The `From Domain`

如果你自定义了 `From Domain`，邮件服务提供商和客户端需要知道 FastComments 已被授权代表你发送邮件。否则，
定义了 `From Domain` 但不遵循下面的步骤，邮件很可能会进入垃圾邮件。

#### 1. Setup SPF

为了允许 FastComments 安全地以你的域名发送邮件，请确保添加允许我们发送的 SPF 记录。

请确保有允许 `mail.fastcomments.com` 和 `sib.fastcomments.com` 以你的域名发送邮件的 SPF 记录。

关于如何操作的更多信息，请参见：https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

除了 SPF 外，你还应设置 DKIM。DNS 配置就绪后，你可以在域配置页面中点击“显示高级”来查看每个域的 DKIM 设置。

你也可以 [调用 API](/guide-api.html#domain-config-structure) 来设置 DKIM 配置。

### Unsubscribe Links

在使用 SSO 时，电子邮件和通知中使用的退订功能可以通过 [via the DomainConfigs API](/guide-api.html#domain-config-structure) 进行自定义。