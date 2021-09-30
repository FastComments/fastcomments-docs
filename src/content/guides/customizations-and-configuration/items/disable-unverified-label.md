[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

By default, FastComments will show an "Unverified Comment" label for comments that have been left for a user that
has an unverified browser session. Read more about unverified commenting [here](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Additionally, this feature can be used, without writing code, in the Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]
