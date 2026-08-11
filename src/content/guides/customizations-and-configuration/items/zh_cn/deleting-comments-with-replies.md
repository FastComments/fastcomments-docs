---
默认情况下，用户可以删除自己的评论。同时，删除他们的评论会自动删除线程中所有子评论和临时评论。此行为也实时生效。

您可以通过以下方式限制此行为：

- 相反，将已删除的评论匿名化（将名称和文本设置为 `[deleted]` 或自定义值）。
- 当有回复时，不允许删除评论。会显示可自定义的错误信息。
- 仅允许管理员和版主在评论有回复时删除。

可以在小部件自定义 UI 的 `Comment Thread Deletion` 部分进行配置。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; alt='在小部件自定义 UI 中的评论线程删除选项，用于匿名化或限制带回复的删除'; title='自定义带回复的删除行为' app-screenshot-end]

---