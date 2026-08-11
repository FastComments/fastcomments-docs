[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

默认情况下，实时评论已启用。这意味着如果有任何评论被添加、删除、编辑或置顶，所有正在查看该评论线程的用户都会同时看到这些更改。

然而，默认情况下，这些新评论会出现在一个动态显示的按钮下，按钮文本类似于“显示 2 条新评论”。

如果新评论是直接回复页面的，按钮会显示在评论线程的顶部。如果它们是对特定评论的回复，按钮则会显示在该评论下方。

这样做是为了防止页面大小不断变化，避免用户在尝试抓取滚动条时产生挫败感。

对于某些使用场景，例如实时竞标或在线活动，这并不是期望的行为——您可能希望评论小部件更像一个“聊天”框，新评论能够“立即显示”。

因此，启用此功能的标志名称为：**showLiveRightAway**。

我们可以按如下方式打开它：

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = '立即显示实时评论'; code-example-end]

这可以在小部件自定义页面上无需代码进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='折叠实时评论设置已切换，新评论会立即出现，而不是在按钮后面'; title='立即显示实时评论' app-screenshot-end]