[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

對於較大型的自訂樣式專案，可能會希望從頭開始，不使用任何預設樣式。

可以透過將 **noStyles** 參數設為 true 來移除所有預設樣式，如下：

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

也可以在 widget 自訂頁面的進階選項中，無需撰寫程式碼即可進行自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]