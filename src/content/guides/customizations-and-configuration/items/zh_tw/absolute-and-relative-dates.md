---
[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

預設會使用本地化的相對日期。例如，在最近留下的評論旁你可能會看到 "11 分鐘前"。

有時你可能需要或希望同時保留此相對日期格式，並在旁邊顯示完整日期，此時可將此參數設為 true。 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

這可以在不撰寫程式碼的情況下於小工具自訂頁面的「進階選項」中進行自定義。你必須先啟用「絕對日期」才能在使用者介面中看到此選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---