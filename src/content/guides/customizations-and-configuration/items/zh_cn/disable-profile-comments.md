[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会在用户个人资料上显示一个“个人资料评论”标签页，允许访客在某人的个人资料上留下评论。

但是，我们可以禁用此标签页：

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = '禁用个人资料评论'; code-example-end]

这也可以在不编写代码的情况下完成。在小部件自定义页面中，查看“禁用个人资料评论”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='小部件自定义页面，已勾选“禁用个人资料评论”复选框以隐藏个人资料评论标签页'; title='禁用个人资料评论' app-screenshot-end]