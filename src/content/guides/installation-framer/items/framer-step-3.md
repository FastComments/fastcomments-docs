The Framer Live Comments FastComments snippet is below.

[inline-code-attrs-start title = 'FastComments Framer-Specific Comments Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
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
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Or, alternatively, you can use the Streaming Chat widget. The Framer Streaming Chat FastComments snippet is:

[inline-code-attrs-start title = 'FastComments Framer-Specific Streaming Chat Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        // Fill these in!
        const PAGE_TITLE = "My Cool Page";
        const PAGE_URL = "My Page URL";

        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo',
                pageTitle: PAGE_TITLE,
                url: PAGE_URL,
                urlId: PAGE_URL
            });
            container.fastCommentsSetup = true;
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

Note that Framer limits the height of widgets and does not support auto-resizing, so we've chosen the Live Chat
widget here since it is fixed height.
