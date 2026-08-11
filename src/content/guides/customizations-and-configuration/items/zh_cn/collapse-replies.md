[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

默认情况下，顶层评论的回复会显示。

可以将其配置为用户必须点击顶层评论上的“Show Replies”才能查看子评论。

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

这可以在小部件自定义页面上无需代码进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='在小部件自定义 UI 中的折叠回复选项，将子评论隐藏在“Show Replies”链接后面'; title='折叠回复' app-screenshot-end]

此设置不会影响最初加载的顶层评论数量。如果您有一个顶层评论以及 29 条子评论，启用此设置后，您将：

- 看到该顶层评论。
- 在该评论下看到“Show Replies (29)”。

如果您希望在使用此选项的同时显示所有顶层评论，请将[起始页设为 -1](#starting-page)设置为。