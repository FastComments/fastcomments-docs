[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Commenting can be locked so no new comments or votes can be left by setting the readonly flag to true.

Comments will also be unable to be edited or deleted.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

This can be customized without code, on the widget customization page, for an entire domain, or page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Update!

As of November 2022, threads can be locked or unlocked **live** by administrators and moderators via the three-dot menu above the reply area.

This will prevent new comments, while still allowing voting, and allowing users to delete their comments if desired, while `readonly` does not allow these things. 

This corresponds to the `isClosed` field in the `Page` API.
