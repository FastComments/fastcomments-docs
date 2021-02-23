[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

When sending notification emails, or rendering comments in user interfaces like the moderation page, it's helpful to be able to link
from the comment to the page it's on.

If URL ID isn't always an ID, then we have to store URL some place else. That's what the "url" property is for, defined as follows.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

A common use case is tying the comment thread to an identifier, like an article, and then linking back to a particular page, for example:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

The URL does not get cleaned of common marketing parameters. By default, whatever the current page URL is, is the URL stored with the comment.
