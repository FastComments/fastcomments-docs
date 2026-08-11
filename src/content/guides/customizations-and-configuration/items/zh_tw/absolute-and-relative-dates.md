[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

預設情況下，會使用本地化的相對日期。例如，在最近留下的評論旁邊，您可能會看到「11 分鐘前」。

可能需要或希望保留此相對日期格式，同時在旁邊顯示完整日期，這時您可以將此參數設為 true。 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = '同時使用絕對與相對日期'; code-example-end]

此設定可在小工具自訂頁面的「進階選項」中，無需撰寫程式碼即可自訂。您必須先啟用「絕對日期」才能在 UI 中看到此選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='小工具自訂頁面的進階選項，已啟用絕對日期與結合的相對日期設定'; title='同時使用絕對與相對日期' app-screenshot-end]