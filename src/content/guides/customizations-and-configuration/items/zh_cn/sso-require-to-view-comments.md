FastComments SSO (<a href="#sso">详细信息</a>) 为您的用户提供了一种无需登录其他平台即可发表评论的方式。

但是，仅此并不能保护您的评论线程，因为默认情况下评论数据是公开信息——任何可以查看页面的人都可以查看评论。

通过更改一个设置，我们可以限制评论的获取，除非是管理员或有效的 SSO 用户，否则无法获取评论。

#### No-Code Setup

当设置了 SSO 时，我们可以通过创建一个 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">自定义规则</a> 来防止查看和与我们的评论线程交互。

在执行此操作时，搜索 SSO，您会找到此选项：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

启用它并保存自定义规则。

#### Only Protect a Certain Domain or Page

要仅保护某个域或页面，我们只需配置自定义规则即可。

在自定义界面的顶部，我们会找到两个输入，域（Domain）和 URL ID。

要仅保护特定域，请在“domain”字段中输入相应的域名。

要保护特定页面，请在“URL ID”字段中输入页面 URL。如果您与 FastComments 有自定义集成，您也可以在此处输入某种类型的 ID 而不是 URL。

#### Security Levels

在要求 SSO 时，您需要决定是要求 Simple SSO 还是 Secure SSO。如果您要求 Simple SSO，则两者都被允许；但如果您要求 Secure SSO，则必须使用通过您的 API 密钥进行哈希处理的 Secure SSO 有效负载来获取内容，才能查看。

当您选择“Require SSO To View Comments”时，将出现安全级别选项。

#### Protection Beyond Reading

启用此选项将保护页面或域，除非用户通过 SSO 登录，否则无法发表评论。

#### Gotchas

在您设置 SSO 集成之前创建评论的任何用户将无法看到这些评论，除非他们通过您的 SSO 集成登录。