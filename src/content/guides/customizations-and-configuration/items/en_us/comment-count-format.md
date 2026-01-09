[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

The comment count displayed at the top of the comment widget can be customized.

This can be replaced with any string, and the value **[count]** will be replaced with the count value, localized for the user.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]