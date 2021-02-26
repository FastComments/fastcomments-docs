[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

By default, FastComments will only ask the user for their comment, their username, and their email.

However, in some situations you may want to have the user leave a link to their own blog or website.

We can enable showing an extra input field to leave the user's website URL by setting the **enableCommenterLinks** flag to true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

When said URL is provided, the user's account will be updated and all of their username on all past and future comments will link to this URL.

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.click-to-show-comments'; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]
