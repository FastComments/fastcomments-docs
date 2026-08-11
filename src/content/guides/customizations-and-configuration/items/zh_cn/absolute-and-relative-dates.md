[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

默认情况下，使用本地化的相对日期。例如，在最近留下的评论旁边，你可能会看到“11 分钟前”。

可能需要或希望保留此相对日期格式，同时在旁边显示完整日期，在这种情况下，你需要将此参数设置为 true。

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = '同时使用绝对日期和相对日期'; code-example-end]

这可以在无需代码的情况下进行自定义，位于小部件自定义页面的“高级选项”下。你需要先启用绝对日期才能在 UI 中看到此选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='小部件自定义页面的高级选项，已启用绝对日期和组合相对日期设置'; title='同时使用绝对日期和相对日期' app-screenshot-end]