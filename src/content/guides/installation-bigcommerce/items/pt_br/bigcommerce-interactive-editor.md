Não é recomendável adicionar o FastComments via o Page Builder do BigCommerce, pois o código teria que ser adicionado manualmente em cada página desejada.

No entanto, se isso for desejado, o trecho de código a seguir deve ser usado. Trechos de código de outros tutoriais não funcionarão devido à natureza do BigCommerce:

[inline-code-attrs-start title = 'Trecho do Page Builder do BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        function attemptLoad() {
            if (loaded) {
                return;
            }
            if (!window.FastCommentsUI) {
                return;
            }
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo"
            });
            loaded = true;
        }
        attemptLoad();
        const interval = setInterval(function () {
            attemptLoad();
            if (loaded) {
                clearInterval(interval);
            }
        }, 300);
    })();
</script>
[inline-code-end]

---