No se recomienda añadir FastComments mediante el Page Builder de BigCommerce, ya que entonces el código debe agregarse manualmente en cada página deseada.

Sin embargo, si esto es lo que desea, debe utilizarse el siguiente fragmento de código. Los fragmentos de código de otros tutoriales no funcionarán debido a la naturaleza de BigCommerce:

[inline-code-attrs-start title = 'Fragmento del Page Builder de BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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