---
默认情况下，FastComments 允许用户删除他们的评论。

但是，可以阻止此操作。

在小部件自定义页面，查看 "Disable Deleting" 选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='在小部件自定义页面的 Disable Deleting 选项，防止评论者删除他们的评论'; title='禁用评论删除' app-screenshot-end]

- 这仅影响普通评论者，而不影响版主或管理员，后者仍然可以删除。
- 当传递 `contextUserId` 时，这也会影响 API 集成。 

---