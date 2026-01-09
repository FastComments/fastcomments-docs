[related-parameter-start name = 'disableSuccessMessage'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在發表評論後顯示成功訊息。可透過下列方式停用：

[code-example-start config = {disableSuccessMessage: true}; linesToHighlight = [6]; title = 'Disable Success Message'; code-example-end]

也可以不用撰寫程式碼，在小工具自訂頁面中進行：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-success-message']; selector = '.disable-success-message'; title='Disable Success Message' app-screenshot-end]