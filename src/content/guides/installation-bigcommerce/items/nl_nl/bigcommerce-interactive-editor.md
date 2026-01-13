---
Het wordt niet aanbevolen om FastComments via de Page Builder van BigCommerce toe te voegen, omdat de code dan handmatig aan elke gewenste pagina moet worden toegevoegd.

Als dit echter gewenst is, moet het volgende codefragment worden gebruikt. Codefragmenten uit andere tutorials werken niet vanwege de aard van BigCommerce:

[inline-code-attrs-start title = 'BigCommerce Page Builder-codefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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