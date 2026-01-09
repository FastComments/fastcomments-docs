[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

默认情况下，使用本地化的相对日期。例如，在最近发布的评论旁你可能会看到 "11 分钟前"。

有时可能需要或希望使用绝对日期，在这种情况下你可以将此参数设置为 true。 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

这可以在小部件自定义页面的高级选项中无需编写代码进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]