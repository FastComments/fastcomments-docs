Se stai cercando lo snippet per i commenti live di ClickFunnels, prova questo:

[inline-code-attrs-start title = 'Snippet per commenti live di ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // alcuni provider modificano lo snippet del codice rendendolo asincrono
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