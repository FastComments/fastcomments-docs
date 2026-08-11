[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 只会要求用户提供评论内容、用户名和电子邮件。

然而，在某些情况下，您可能希望用户留下指向其个人博客或网站的链接。

我们可以通过将 **enableCommenterLinks** 标志设置为 true，来启用显示一个额外的输入字段，以填写用户的网站 URL：

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

当提供该 URL 时，用户的账户将被更新，且其过去和未来所有评论中的用户名都将链接到该 URL。

这可以在小部件自定义页面上无需编写代码进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='小部件自定义页面，已勾选评论者链接复选框，以在评论表单中添加网站 URL 字段'; title='启用评论者链接' app-screenshot-end]