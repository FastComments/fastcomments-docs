[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

By default, FastComments will allow the user to enter a comment of as many lines as they like, up to the default character limit.

However, it may be desirable to limit the user to entering only a single line of text. Some example use cases include online bidding, or live chat, which FastComments
can be used for.

We enable the **useSingleLineCommentInput** flag as follows:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

This can also be done without code. In the widget customization page, see the "Enable Single-Line Comment Input" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Note that, the comments on each page for each sort direction are pre-computed, so all sort directions have the same performance.