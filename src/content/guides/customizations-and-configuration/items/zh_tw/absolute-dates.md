[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

預設會使用在地化的相對日期。例如，在一則最近留下的評論旁你可能會看到 "11 分鐘前"。

在某些情況下可能需要或希望使用絕對日期，此時將此參數設為 true。 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

這可以在不撰寫程式碼的情況下自訂，於小工具自訂頁面中的進階選項：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]

---