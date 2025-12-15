[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

By default, replies to top level comments show.

This can be configured so that the user has to click "Show Replies" on the top-level comments to see the children.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

This setting will not affect the number of top-level comments initially loaded. If you have one top level comment, and 29 children, with this setting on you will:

- See the top level comment.
- See Show Replies (29) under this comment.

If you wish to show all top level comments in combination with this option, set [starting page to -1](#starting-page).
