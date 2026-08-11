[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

对于身份验证，FastComments 依赖浏览器中启用第三方 Cookie。若未启用，用户必须始终留下电子邮件才能发表评论（除非隐藏了电子邮件输入字段），并且他们的评论默认会显示为未验证。

为了解决此问题，您可以启用第三方 Cookie 绕过。 

启用此设置后，会出现一个小弹窗，显示用户正在登录的消息。该弹窗会在用户与评论小部件交互时出现，例如当他们留下评论时。

我们可以通过在代码中将 **enableThirdPartyCookieBypass** 标志设为 true 来实现：

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = '启用第三方 Cookie 绕过'; code-example-end]

我们也可以通过小部件自定义 UI 设置此项，位于 `Enable Third-Party Cookie Popup` 下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='已勾选“Enable Third-Party Cookie Popup”复选框的小部件自定义页面'; title='启用第三方 Cookie 绕过' app-screenshot-end]