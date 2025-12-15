FastComments also supports the Page Reacts (aka Floating Like button) widget for Hostinger.

You can see it in action in the bottom right of this page!

### Note!

These instructions are for the Hostinger Site Builder. If you're using Hostinger *WordPress*, then just grab the below code, and add it to your WordPress site
using [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), which is a free and easy plugin for adding small code snippets to your site.

1. First, grab the code:

[inline-code-attrs-start title = 'Hostinger Floating Likes Code'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Then, in Hostinger, open the site builder.
3. Go to Website Settings in the bottom Left.
4. Select Integrations.
5. Add the new code to the *end* of the `Custom code` field, and publish your site.
6. You will not see the widget in preview mode, but it will appear on the published version of the site.
