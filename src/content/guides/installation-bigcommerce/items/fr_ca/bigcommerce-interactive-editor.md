Il n'est pas recommandé d'ajouter FastComments via le Page Builder de BigCommerce, car le code doit ensuite être ajouté manuellement à chaque page souhaitée.

Cependant, si cela est souhaité, l'extrait de code suivant doit être utilisé. Les extraits de code provenant d'autres tutoriels ne fonctionneront pas en raison de la nature de BigCommerce :

[inline-code-attrs-start title = 'Extrait pour le Page Builder de BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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