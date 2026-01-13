Si buscas el fragmento de código de comentarios en vivo de ClickFunnels, prueba esto:

[inline-code-attrs-start title = 'Fragmento de comentarios en vivo de ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // algunos proveedores cambian el fragmento de código para que sea asíncrono
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