[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

By default, FastComments will render the comment input box and comment thread at the same time. To save some vertical space,
it will also hide any other required fields until the widget is interacted with.

However, the comment widget can be hidden behind a button, for example:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

The button uses different translated text based on whether the comments are currently shown or not. If the comments are hidden, it uses `translations.SHOW_COMMENTS_BUTTON_TEXT`. If the
comments are shown, it uses `translations.HIDE_COMMENTS_BUTTON_TEXT`. The translations can contain the text `[count]` which will
be replaced by the localized count.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

This is designed to replace the `hideCommentsUnderCountTextFormat` configuration.

The count is updated live with the comment thread. The button is not shown if there are no comments.

This can be enabled without code by creating a customization rule and enabling "Click to Show Comments":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]