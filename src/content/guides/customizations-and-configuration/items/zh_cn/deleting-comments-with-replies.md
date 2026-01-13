默认情况下，用户可以删除他们自己的评论。此外，删除他们的评论会自动
删除线程中该评论的所有子评论和临时评论。此行为也是实时生效的。

您可以通过以下方式限制此行为：

- 作为替代，将已删除的评论匿名化（将 name 和 text 设置为 `[deleted]` 或自定义值）。
- 不允许在存在回复时删除评论。会显示可自定义的错误消息。
- 当评论有回复时，仅允许管理员和版主删除评论。

可以通过小部件自定义 UI 中的 `Comment Thread Deletion` 部分进行配置。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]