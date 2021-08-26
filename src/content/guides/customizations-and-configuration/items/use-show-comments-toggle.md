[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

By default, FastComments will render the comment input box and comment thread at the same time. To save some vertical space,
it will also hide any other required fields until the widget is interacted with.

However, the comment widget can be hidden behind a button.

The button uses different translated text based on whether
the comments are currently shown or not. If the comments are hidden, it uses `translations.SHOW_COMMENTS_BUTTON_TEXT`. If the
comments are shown, it uses `translations.HIDE_COMMENTS_BUTTON_TEXT`. The translations can contain the text `[count]` which will
be replaced by the localized count.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

This is designed to replace the `hideCommentsUnderCountTextFormat` configuration.

The count is updated live with the comment thread.

