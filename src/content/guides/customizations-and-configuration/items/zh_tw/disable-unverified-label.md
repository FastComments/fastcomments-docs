[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在留給使用未經驗證瀏覽器工作階段的使用者的評論上顯示「未驗證評論」標籤。關於未驗證評論的更多資訊請見 [here](https://docs.fastcomments.com/guide-comment-vote-verification.html)。

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

此外，這項功能也可以在自訂化 UI 中使用，而無需撰寫程式碼：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]