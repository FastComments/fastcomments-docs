[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

預設情況下，使用本地化的相對日期。例如，在最近發表的評論旁邊，您可能會看到「11 分鐘前」。

可能需要或希望使用絕對日期，在這種情況下，您需要將此參數設為 true。 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

這可以在小工具自訂頁面的「進階選項」中，無需編寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='小工具自訂頁面的「進階選項」，已開啟絕對日期切換'; title='使用絕對日期' app-screenshot-end]