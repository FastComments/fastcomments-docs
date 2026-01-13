[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

By default, localized relative dates are used. For example, next to a recently posted comment you may see "11 minutes ago".

You may need or want to use absolute dates; in that case, set this parameter to true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

This can be customized without writing code on the widget customization page, under Advanced Options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]