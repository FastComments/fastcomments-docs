---
默认情况下，FastComments 将允许用户编辑他们的评论。

但是，可以阻止此操作。

在小部件自定义页面，请查看 “禁用编辑” 选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='禁用评论编辑' app-screenshot-end]

- 这仅影响普通评论者，不影响版主或管理员，他们仍然可以编辑。
- 当传入 `contextUserId` 时，这也会影响 API 集成。

---