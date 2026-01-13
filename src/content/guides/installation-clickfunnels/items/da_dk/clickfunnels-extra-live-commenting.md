---
Hvis du leder efter ClickFunnels Live Commenting-kodesnippet, prøv dette:

[inline-code-attrs-start title = 'ClickFunnels Live Commenting-kodesnippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="min-width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // nogle udbydere ændrer kodesnutten til at være asynkron
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

---