[related-parameter-start name = 'hideCommentsUnderCountTextFormat'; type = 'string'; related-parameter-end]

By default, FastComments will render the comment input box and comment thread at the same time. To save some vertical space,
it will also hide any other required fields until the widget is interacted with.

However, the comment widget can be hidden behind a button, with customized text using **hideCommentsUnderCountTextFormat**.

This string value can also contain a **[count]** token, which will be replaced with the localized comment count.

[code-example-start config = {hideCommentsUnderCountTextFormat: "Show [count] Comments"}; linesToHighlight = [6]; title = 'Click to Show Comments'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.click-to-show-comments'; selector = '.click-to-show-comments'; title='Click to Show Comments' app-screenshot-end]
