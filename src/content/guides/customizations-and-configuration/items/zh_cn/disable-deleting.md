默认情况下，FastComments 会允许用户删除他们的评论。

但是，可以阻止此操作。

在小部件自定义页面中，查看“禁用删除”选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- 这只影响普通评论者，不影响版主或管理员，他们仍然可以删除。
- 这也会影响 API 集成，在传入 `contextUserId` 时生效.