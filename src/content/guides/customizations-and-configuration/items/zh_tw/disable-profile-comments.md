---
[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在使用者個人檔案上顯示「個人檔案留言」分頁，允許訪客在某人的個人檔案上留下留言。

不過，我們可以停用此分頁：

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Disable Profile Comments'; code-example-end]

這也可以不透過程式完成。在小工具自訂頁面，請參閱「停用個人檔案留言」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; title='Disable Profile Comments' app-screenshot-end]

---