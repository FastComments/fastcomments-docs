FastComments SSO (<a href="#sso">此处详情</a>) 为您的用户提供了一种无需登录其他平台即可发表评论的方式。

然而，仅此并不能保护您的评论线程，因为默认情况下评论数据是公开信息——任何能够查看页面的人都可以看到评论。

通过更改设置，我们可以限制只有管理员或有效的 SSO 用户才能获取评论。

#### No-Code Setup

当 SSO 设置完成后，我们可以通过创建一个 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">自定义规则</a> 来防止查看和交互我们的评论线程。

操作时，搜索 SSO，即可找到此选项：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='在自定义规则中启用了“需要 SSO 查看评论”选项，并可选择安全级别'; title='Require SSO To View Comments' app-screenshot-end]

启用后保存自定义规则。

#### Only Protect a Certain Domain or Page

要仅保护特定的域名或页面，只需相应地配置自定义规则即可。

在自定义 UI 顶部，我们会看到两个输入框，Domain（域名）和 URL ID（页面标识）。

若只保护特定域名，请在 “domain” 字段中输入相应的域名。

若保护特定页面，请在 “URL ID” 字段中输入页面 URL。如果您使用 FastComments 的自定义集成，也可以在此处输入一种 ID 而非 URL。

#### Security Levels

在要求 SSO 时，您需要决定是使用 Simple SSO 还是 Secure SSO。若选择 Simple SSO，则两者均被允许；若选择 Secure SSO，则内容必须使用使用您 API 密钥哈希后的 Secure SSO 负载获取，才能被查看。

当您选择 “Require SSO To View Comments” 时，安全级别选项将会出现。

#### Protection Beyond Reading

启用此选项后，页面或域名将只能在用户通过 SSO 登录后才能发表评论。

#### Gotchas

在您集成 SSO 之前创建的评论的用户，将无法看到这些评论，除非他们通过您的 SSO 集成登录。