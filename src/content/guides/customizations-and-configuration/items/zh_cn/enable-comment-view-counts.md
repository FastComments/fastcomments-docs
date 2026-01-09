[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 不会跟踪谁查看了每条评论，也不会提供任何相关统计数据。

但是，我们可以启用此功能，系统将在每个用户滚动到评论时开始进行跟踪。

发生这种情况时，每条评论旁边显示的眼睛图标旁的计数将会增加。计数会实时更新，并根据用户的区域设置以缩写形式显示。

我们可以通过将 **enableViewCounts** 标志设置为 true 来启用此功能：

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

这可以在小部件自定义页面上无需编码即可自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

我们会跟踪查看评论的用户 ID*，因此如果你再次查看该评论，计数不会增加。如果在两年后再次查看，计数将会再次增加。

- *注：或匿名会话 ID，或用户的 IP（哈希值）。