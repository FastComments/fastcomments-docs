[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

By default, FastComments will display user badges only on their comments within the comment thread.

However, we can show user badges next to their name above the comment form by enabling this feature in the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

This will display the user's badges alongside their name in the top bar area, making their achievements and status more prominent when they're composing a comment.

Note that this feature must be enabled in the widget customization UI to work. You can optionally set the **showBadgesInTopBar** flag to false in your code configuration to selectively disable it even when it's turned on at the server level:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]