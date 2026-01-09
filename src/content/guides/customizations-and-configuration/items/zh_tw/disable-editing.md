---
預設情況下，FastComments 允許使用者編輯他們的留言。

不過，可以防止此行為。

在 widget 自訂頁面，請查看 "停用編輯" 選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- 這只會影響一般的留言者（Commenters），不會影響版主或管理員，他們仍然可以編輯。
- 當傳遞 `contextUserId` 時，這也會影響 API 整合。 

---