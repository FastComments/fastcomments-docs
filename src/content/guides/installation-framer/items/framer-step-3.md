The Framer-specific FastComments snippet is:

[inline-code-attrs-start title = 'FastComments Framer-Specific Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<style>
#main > div:first-child {
    height: auto !important;
}
</style>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                urlId: window.location.path
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

FastComments supports the Framer editor, so you should see something like this once you paste the code in (you might have to click `Publish`):

<div class="screenshot white-bg">
    <div class="title">Comment Widget Preview</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Comment Widget Preview" />
</div>

Now when you view your site you should see the comment area! In the sidebar of Framer you can set the widget as full width as well, if desired.
