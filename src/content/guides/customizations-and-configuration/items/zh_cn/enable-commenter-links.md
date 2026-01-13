[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 只会要求用户提供他们的评论、用户名和电子邮件。

但是，在某些情况下，您可能希望用户留下指向他们自己博客或网站的链接。

我们可以通过将 **enableCommenterLinks** 标志设置为 true 来启用显示额外的输入字段以填写用户的网站 URL：

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

当提供该 URL 时，用户的账户将被更新，其在所有过去和将来的评论中显示的用户名都会链接到该 URL。

这可以在小部件自定义页面上无需编写代码即可自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]