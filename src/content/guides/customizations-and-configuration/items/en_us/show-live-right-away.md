[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

By default, live commenting is enabled. This means that if any comments are added, deleted, edited, or pinned, the changes should appear
to all users viewing the comment thread at the same time.

However, by default those new comments will appear under a dynamically shown button with the text similar to "Show 2 New Comments".

If the new comments are replies directly to the page, the button will show at the top of the comment thread. If they are replies to a particular comment, 
the button will show under that comment.

This is to prevent the page size changing constantly on the user, potentially causing frustration when trying to grab the scroll bar.

For some use cases, like live bidding, or online events, this is not the desired behavior - you may want the commenting widget to be
more like a "chat" box where new comments "show right away".

Hence, the name of the flag that enables that feature: **showLiveRightAway**.

We can turn it on as follows:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]