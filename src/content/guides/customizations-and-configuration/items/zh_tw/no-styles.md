[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

對於較大的自訂樣式專案，可能希望從零開始，完全不使用預設樣式。

可以透過將 **noStyles** 參數設為 true 來移除所有預設樣式，如下所示：

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

這可以在小工具自訂頁面的「進階選項」中，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='在小工具自訂頁面的「進階選項」下啟用的「停用所有預設樣式」核取方塊'; title='停用所有預設樣式' app-screenshot-end]