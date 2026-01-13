[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

By default, the FastComments widget will resize itself vertically to fit all visible comments. Pagination is achieved via a "View Next"
button at the end of the current page, as we have found this to be the interaction that feels the nicest for most users.

However, there are some cases where infinite scrolling is preferred. For example, we use this feature in our Stream Chat product.

We can hide the "View Next" buttons and switch to infinite scrolling by setting the **enableInfiniteScrolling** flag to true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

This also requires adding custom CSS. Add custom CSS for the `.comments` selector to enable scrolling, for example:

[inline-code-attrs-start title = 'Enable Infinite Scrolling'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

A full working example would be:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

In the above example we use the `customCSS` property, however it is suggested the Widget Configuration UI is used instead for performance reasons. [See the Custom CSS documentation.](/guide-customizations-and-configuration.html#custom-css)