[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

By default, if a user types a comment and then refreshes the page, closes the tab, or navigates away before submitting, the draft is lost silently.

Setting **warnOnUnsavedComment** to true makes the browser ask the user to confirm before leaving the page while any comment box, or an edit in progress, still contains text. Once the comment is submitted the text is cleared, so no prompt is shown.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Warn On Unsaved Comment'; code-example-end]

The prompt uses the browser's own dialog. Modern browsers show their own wording and ignore custom text, so the message cannot be customized.

This option loads a small extension on demand, so it adds nothing to the widget for sites that do not enable it.
