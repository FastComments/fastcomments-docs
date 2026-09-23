[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 不会跟踪谁查看了每条评论，也不会提供任何相关统计。

但是，我们可以启用此功能，系统随后将在每位用户滚动到评论时开始进行跟踪。

当发生这种情况时，每条评论旁边的眼睛图标旁的计数会递增。计数会实时更新，并根据用户的语言环境进行缩写。

我们可以通过将 **enableViewCounts** 标志设置为 true 来启用此功能：

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

这可以在无需代码的情况下，通过小部件自定义页面进行设置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='小部件自定义页面，已勾选查看计数复选框，使每条评论显示眼睛图标和计数'; title='启用评论查看计数' app-screenshot-end]

我们会跟踪查看评论的用户 ID*，保留一周时间，这样如果您在一周内再次查看该评论，计数不会递增。若在一周后再次查看，计数将再次递增。

- *注意：也可以是匿名会话 ID，或用户的 IP（哈希值）。