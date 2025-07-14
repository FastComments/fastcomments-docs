Click on the HTML element you just added. In the property editor that appears, paste the following code into the HTML field:

[inline-code-attrs-start title = 'Bubble.io Live Commenting Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // bubble tends to change the code snippet to be async
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
                url: top.location.href,
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

<div class="screenshot white-bg">
    <div class="title">Insert FastComments Code</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Inserting FastComments Code into HTML Element" />
</div>

Note: This code includes a retry mechanism to ensure FastComments loads properly in Bubble's dynamic environment.
Other code snippets will not work.

Remember to replace `'demo'` with your actual FastComments tenant ID after signing up. If you're logged in to FastComments.com, it should already be replaced.
