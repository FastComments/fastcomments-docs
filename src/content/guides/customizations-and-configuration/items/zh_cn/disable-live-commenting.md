[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 将启用实时评论。

这意味着评论线程的每位查看者都应看到相同的内容。

例如，如果添加了评论，该评论应显示。如果评论被编辑或删除，
那么这些评论会对线程的所有查看者被相应地编辑或删除。投票以及所有审核操作亦是如此。

但是，我们可以禁用此功能：

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

这也可以在无需编写代码的情况下完成。在小部件自定义页面中，参阅 "禁用实时评论" 部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]