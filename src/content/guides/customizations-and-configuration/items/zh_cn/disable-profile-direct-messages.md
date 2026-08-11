[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会在用户个人资料上显示一个“直接消息”标签，允许访客向用户发送直接消息。

但是，我们可以禁用此标签：

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

这也可以在不编写代码的情况下完成。在小部件自定义页面中，查看“禁用直接消息”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='小部件自定义页面，已勾选“禁用直接消息”复选框以隐藏个人资料消息标签'; title='禁用个人资料直接消息' app-screenshot-end]