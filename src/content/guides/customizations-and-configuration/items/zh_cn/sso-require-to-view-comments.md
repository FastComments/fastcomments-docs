FastComments SSO (<a href="#sso">此处详情</a>) 为您的用户提供了一种无需登录其他平台即可发表评论的方式。

然而，仅此并不能保护您的评论线程，因为默认情况下评论数据是公开可用的信息——任何能够查看页面的人都可以查看评论。

通过更改设置，我们可以限制评论的获取，除非是管理员或有效的 SSO 用户。

#### 无代码设置

当 SSO 设置完成后，我们可以通过创建<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">自定义规则</a>来阻止查看和交互我们的评论线程。

操作时，搜索 SSO，您将看到此选项：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='在自定义规则中启用了“需要 SSO 才能查看评论”选项，并选择安全级别'; title='需要 SSO 才能查看评论' app-screenshot-end]

启用后保存自定义规则。

#### 仅保护特定域名或页面

若只想保护特定域名或页面，只需相应配置自定义规则即可。

在自定义 UI 顶部，我们会看到两个输入框，Domain（域名）和 URL ID（页面标识）。

要仅保护特定域名，请在 “domain” 字段中输入相应的域名。

要保护特定页面，请在 “URL ID” 字段中输入页面的 URL。如果您使用 FastComments 的自定义集成，也可以在此处输入一种 ID 类型而非 URL。

#### 安全级别

在要求 SSO 时，您需要决定是使用 Simple SSO 还是 Secure SSO。若选择 Simple SSO，则两者均被允许；若选择 Secure SSO，则内容必须使用使用您 API 密钥哈希的 Secure SSO 负载来获取，才能被查看。

当您选择 “需要 SSO 才能查看评论” 时，安全级别选项会出现，并且仅在该选项开启时生效。

<a href="#sso-simple">通过 Simple SSO 自动验证评论并允许 Simple SSO 管理员</a> 的设置是独立的，无论此选项是否开启，都将生效。

#### 超出阅读的保护

启用此选项后，页面或域名将只能在用户通过 SSO 登录后才能进行评论。

#### 注意事项

在您集成 SSO 之前已创建的评论的用户，将无法看到这些评论，除非他们通过您的 SSO 集成登录。