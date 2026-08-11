[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

默认情况下，使用本地化的相对日期。例如，在最近留下的评论旁边，你可能会看到“11 分钟前”。

可能需要或希望使用绝对日期，在这种情况下，你需要将此参数设置为 true。 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = '使用绝对日期'; code-example-end]

这可以在无需代码的情况下进行自定义，前往小部件自定义页面的“高级选项”。:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='小部件自定义页面的高级选项，已打开绝对日期切换'; title='使用绝对日期' app-screenshot-end]