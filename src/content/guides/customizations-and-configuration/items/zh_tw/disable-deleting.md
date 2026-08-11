---
預設情況下，FastComments 允許使用者刪除他們的評論。

然而，也可以防止此行為。

在小工具自訂頁面中，請查看「Disable Deleting」選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='在小工具自訂頁面上的 Disable Deleting 選項，防止評論者刪除他們的評論'; title='停用評論刪除' app-screenshot-end]

- 這僅影響一般評論者，而不影響版主或管理員，後者仍然可以刪除。
- 這也會影響在傳遞 `contextUserId` 時的 API 整合。 

---