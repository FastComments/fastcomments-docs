[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

By default, localized relative dates are used. For example, next to a recently posted comment you might see "11 minutes ago".

If you want to keep the relative date format and also show the full date next to it, set this parameter to true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

You can customize this without writing code on the widget customization page, under Advanced Options. You must first enable Absolute Dates to see this option in the UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]