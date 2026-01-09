Non è consigliato aggiungere FastComments tramite il Page Builder di BigCommerce, perché il codice dovrà poi essere aggiunto manualmente a ogni pagina desiderata.

Tuttavia, se si desidera procedere in questo modo, è necessario utilizzare lo snippet di codice seguente. Gli snippet di codice di altri tutorial non funzioneranno a causa della natura di BigCommerce:

[inline-code-attrs-start title = 'Snippet per il Page Builder di BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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