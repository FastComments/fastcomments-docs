[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

By default, FastComments does not track who viewed each comment or provide any stats around this.

However, we can enable this feature, and then the system will start to track as each user scrolls to a comment.

When this happens, a count next to an eye icon shown on each comment will be incremented. The count is updated live and abbreviated according to the user's locale.

We can enable this by setting the **enableViewCounts** flag to true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]
