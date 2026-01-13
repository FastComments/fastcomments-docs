[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

By default, FastComments renders voting options as up and down arrows, allowing users to upvote or downvote a comment.

However, you can change the style of the vote toolbar. The current options are the default Up/Down buttons or a Heart-style voting mechanism.

We use the **voteStyle** flag as follows:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

We strongly recommend doing this without code, as it also enables server-side validations. On the widget customization page, see the "Vote Style" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Voting can also be disabled; see `Disable Voting` above the style options.

---