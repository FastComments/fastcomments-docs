[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

When rendering a comment thread, or leaving a comment, FastComments needs to know what page, or article, or product
those comments belong to.

To do this, we use something we call the "URL ID". It's either an identifier, like a string or a number, or a URL.

By default, if you do not specify the urlId, it will become the page URL. We will take the current page URL, and clean it to remove
any common marketing parameters or tracking identifiers.

In the case of third party integrations, like WordPress, our plugin will usually use the identifier that represents the current information being viewed as
the URL ID, for example the article/page id.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

One thing that we'll often reference in this document is the <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Customization UI</a>.

This UI can be used to make many changes to the comment widget without using code.

When creating a customization rule, we'll often want it to apply to all pages to our site. However, in some cases we want to customize the comment widget
on a particular page, either to apply custom styling, or maybe make comments for that particular page anonymous. You could also, for example, have live comments
appear right away on some pages, while hiding them under notification buttons on others.

This is all possible via the URL ID input field on this page, which looks like as follows:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

The value in this field should match the *urlId* parameter passed into the comment widget. If you want your customization rule to be *urlId* agnostic, leave this field empty or enter *.

As of 2023 the `URL ID` field in widget customization now also takes patterns! For example you may
have `*/blog/*` to add styling specific to your blog and `*/store/*` to have styling specific to your store,
all while using the same domain.

### Gotchas

1. If your page has hash parameters (like example.com#page-1) - this will become part of the URL ID, by default.
2. During migrations, for example from WordPress to Gatsby, you may have to migrate the URL ID comment values after the initial migration. For that, reach out to us.