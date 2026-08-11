[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在使用者個人檔案上顯示「直接訊息」分頁，允許訪客向使用者發送直接訊息。

然而，我們可以停用此分頁：

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = '停用個人檔案直接訊息'; code-example-end]

這也可以在不撰寫程式碼的情況下完成。在小工具自訂頁面中，請參閱「停用直接訊息」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='小工具自訂頁面，已勾選「停用直接訊息」核取方塊，以隱藏個人檔案訊息分頁'; title='停用個人檔案直接訊息' app-screenshot-end]