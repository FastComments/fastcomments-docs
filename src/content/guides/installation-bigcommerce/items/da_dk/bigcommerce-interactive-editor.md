Det anbefales ikke at tilføje FastComments via BigCommerce Page Builder, da koden så skal tilføjes manuelt til hver ønsket side.

Hvis det ønskes, skal følgende kodeudsnit bruges. Kodeudsnit fra andre vejledninger vil ikke fungere på grund af, hvordan BigCommerce fungerer:

[inline-code-attrs-start title = 'BigCommerce Page Builder-kodeudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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