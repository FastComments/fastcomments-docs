[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

默认情况下，会使用本地化的相对日期。例如，在一条最近发表的评论旁，您可能会看到 "11 分钟前"。

有时您可能需要或希望保留这种相对日期格式，同时在旁边显示完整日期，在这种情况下，将此参数设置为 true。 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

此项可在小部件自定义页面的“高级选项”中无需编写代码即可自定义。您需要先启用“绝对日期”，才能在界面中看到此选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]