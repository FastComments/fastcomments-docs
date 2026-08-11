---
[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在使用者個人檔案上顯示「個人檔案評論」分頁，允許訪客在某人的個人檔案上留下評論。

然而，我們可以停用此分頁：

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = '停用個人檔案評論'; code-example-end]

也可以不寫程式碼完成此操作。在小工具自訂頁面中，請參閱「停用個人檔案評論」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='小工具自訂頁面，已勾選「停用個人檔案評論」核取方塊以隱藏個人檔案評論分頁'; title='停用個人檔案評論' app-screenshot-end]

---