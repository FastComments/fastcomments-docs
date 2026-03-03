[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在使用者個人檔案上顯示「私訊」分頁，允許訪客向使用者傳送私訊。

不過，我們可以停用此分頁：

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

也可以不使用程式碼。在小工具自訂頁面，請參閱「停用私訊」部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; title='Disable Profile Direct Messages' app-screenshot-end]

---