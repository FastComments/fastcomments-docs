[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

默认情况下，会显示对顶级评论的回复。

可以配置为用户必须点击顶级评论上的 "显示回复" 才能查看子评论。

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

可以在小部件自定义页面上无需编写代码进行此项自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

该设置不会影响初始加载的顶级评论数量。如果你有一个顶级评论和 29 个子评论，在启用此设置时你将：

- 看到顶级评论。
- 在该评论下看到 "显示回复 (29)"。

如果你希望在启用此选项的同时显示所有顶级评论，请将 [起始页面设为 -1](#starting-page)。