[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

默认情况下，实时评论是启用的。这意味着如果有任何评论被添加、删除、编辑或置顶，这些更改应同时显示
给查看该评论线程的所有用户。

但是，默认情况下，这些新评论将显示在一个动态出现的按钮下，按钮文本类似于 "Show 2 New Comments"。

如果新评论是直接回复页面，按钮会显示在评论线程的顶部。如果它们是回复某条特定评论， 
按钮会显示在该评论下方。

这样做是为了防止页面大小不断变化，可能在用户尝试抓住滚动条时造成挫败感。

对于某些用例，例如实时竞价或在线活动，这并不是期望的行为 - 您可能希望评论小部件更像一个 "chat" 框，新评论可以 "show right away"。

因此，用于启用该功能的标志名称为：**showLiveRightAway**。

我们可以如下开启：

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

无需编码即可在小部件自定义页面上进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]