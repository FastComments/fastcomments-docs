To use FastComments SSR, the client may fetch HTML from the `https://fastcomments.com/ssr/comments` endpoint.

This can be done a number of ways.

### With WordPress

SSR is enabled by default for users without JS enabled as a fallback in the WordPress plugin since version `3.10.2`.

### In a Webpage

With an already existing application, SSR can be added with the [following example](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), assuming the language used is PHP:

[inline-code-attrs-start title = 'PHP-Based SSR Example'; type = 'php'; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

We can also only show the SSR UI when the user has JS disabled:

[inline-code-attrs-start title = 'PHP-Based SSR Fallback Example'; type = 'php'; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

For an example using SSO, [see here](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### With Pre-Rendered Content

Our blog is generated at build time, and provides a [good example of SSR with Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### The Basic Parameters

The basic parameters that you need to pass are:
- `tenantId` - This identifies you as a customer.
- `urlId` - This identifies the page or article to load comments for, and defines where they are saved.
- `url` - This is used for notifications and related features to link back to the comment thread.

### Custom Styling

The SSR version of the comment widget uses the same structure and rendering engine as the JavaScript one.

As such, all custom styling that works for the JavaScript commenting widget works for SSR. 

### Notes

With SSR, there is no JavaScript to control the height of the rendered container. In browsers, a vertical scrollbar may show for long discussions.

As such, you must tune this as desired.
