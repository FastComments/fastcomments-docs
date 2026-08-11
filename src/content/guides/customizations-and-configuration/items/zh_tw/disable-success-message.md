[related-parameter-start name = 'disableSuccessMessage'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在發表評論後顯示成功訊息。可以透過以下方式停用：

[code-example-start config = {disableSuccessMessage: true}; linesToHighlight = [6]; title = '停用成功訊息'; code-example-end]

也可以不使用程式碼完成此操作。在小工具自訂頁面：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-success-message']; selector = '.disable-success-message'; alt='已勾選「停用成功訊息」核取方塊以隱藏發表評論後確認訊息的小工具自訂頁面'; title='停用成功訊息' app-screenshot-end]