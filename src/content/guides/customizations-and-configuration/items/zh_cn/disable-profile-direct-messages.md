---
[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会在用户资料上显示一个“私信”选项卡，允许访客向该用户发送私信。

不过，我们可以禁用此选项卡：

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

也可以在不使用代码的情况下完成。在小部件自定义页面，查看“禁用私信”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; title='Disable Profile Direct Messages' app-screenshot-end]

---