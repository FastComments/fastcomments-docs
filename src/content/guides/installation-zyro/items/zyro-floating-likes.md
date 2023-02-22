FastComments also supports the Page Reacts (aka Floating Like button) widget for Zyro.

You can see it in action in the bottom right of this page!

1. First, grab the code:

[inline-code-attrs-start title = 'Zyro Floating Likes Code'; type = 'bash'; useDemoTenant = true; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating !== undefined) {
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

2. Then, in Zyro, open the site builder.
3. Go to Website Settings in the bottom Left.
4. Select Integrations.
5. Add the new code to the *end* of the `Custom code` field, and publish your site.
6. You will not see the widget in preview mode, but it will appear on the published version of the site.
