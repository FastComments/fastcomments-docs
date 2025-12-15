[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

By default, localized relative dates are used. For example, next to a recently left comment you may see "11 minutes ago".

It may be necessary or desired to keep this relative date format, but also show the full date alongside it, in which case you set this parameter to true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

This can be customized without code, on the widget customization page, under Advanced Options. You will first have to enable Absolute Dates to see this option in the UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]
