[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

By default, FastComments will only ask the user for their comment, username, and email.

However, in some cases you may want the user to provide a link to their blog or website.

You can enable an extra input field for the user's website URL by setting the **enableCommenterLinks** flag to true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

When that URL is provided, the user's account will be updated and their username on all past and future comments will link to that URL.

This can be configured without code on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---