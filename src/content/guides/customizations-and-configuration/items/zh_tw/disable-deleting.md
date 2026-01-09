---
預設情況下，FastComments 會允許使用者刪除他們的留言。

不過，可以阻止這個行為。

在小工具自訂頁面，請查看「停用刪除」選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- 這只會影響一般的評論者（Commenters），不會影響版主或管理員；版主和管理員仍然可以刪除。
- 當傳入 `contextUserId` 時，這也會影響 API 整合。 

---