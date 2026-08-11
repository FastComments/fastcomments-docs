[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 将启用实时评论。

这意味着评论线程的每个观看者都应看到相同的内容。

例如，如果添加了一条评论，该评论应显示。如果评论被编辑或删除，
则这些评论会对线程的所有观看者进行相应的编辑或删除。投票以及所有的审核操作也是如此。

但是，我们可以禁用此功能：

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

这也可以在不使用代码的情况下完成。在小部件自定义页面，查看“禁用实时评论”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='小部件自定义页面的“禁用实时评论”部分，关闭实时线程更新'; title='禁用实时评论' app-screenshot-end]