### How Comments are Tied to Pages and Articles

TODO ability to show examples in all supported libraries - how can we generate those templates?

When rendering a comment thread, or leaving a comment, FastComments need to know what page, or article, or product
those comments belong to.

To do this, we use something we call the "URL ID". It's either an identifier, like a string or a number, or a URL.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6] code-example-end]

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id' app-screenshot-end]
