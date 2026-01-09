[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

对于身份验证，FastComments 依赖于浏览器中启用的第三方 cookie。没有它们，用户将始终需要
留下他们的电子邮件来发表评论（除非电子邮件输入字段被隐藏），并且他们的评论默认将始终显示为未验证。

要解决此问题，您可以启用第三方 cookie 绕过。 

启用此设置后，会弹出一个小窗口，显示一条消息，说明用户正在登录。此弹出窗口
会在用户与评论小部件交互时显示；例如，当他们发表评论时。

我们可以通过在代码中将 **enableThirdPartyCookieBypass** 标志设置为 true 来实现此操作：

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

我们也可以通过小部件自定义 UI，在 `Enable Third-Party Cookie Popup` 下进行此设置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---