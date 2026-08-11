[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會為留下給未驗證瀏覽器會話的使用者的評論顯示「未驗證評論」標籤。欲了解更多關於未驗證評論的資訊，請點擊[此處](https://docs.fastcomments.com/guide-comment-vote-verification.html)。

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

此外，您可以在自訂 UI 中使用此功能，無需撰寫程式碼：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='小工具自訂頁面，已勾選「停用未驗證評論標籤」核取方塊'; title='停用未驗證標籤' app-screenshot-end]