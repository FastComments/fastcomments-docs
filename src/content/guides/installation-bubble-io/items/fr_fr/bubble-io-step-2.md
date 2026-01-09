---
Cliquez sur l'élément HTML que vous venez d'ajouter. Dans l'éditeur de propriétés qui apparaît, collez le code suivant dans le champ HTML :

[inline-code-attrs-start title = 'Extrait de code pour commentaires en direct Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble a tendance à modifier l'extrait de code pour le rendre asynchrone
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
    <div class="title">Insérer le code FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Insertion du code FastComments dans l'élément HTML" />
</div>

Remarque : Ce code inclut un mécanisme de réessai pour garantir que FastComments se charge correctement dans l'environnement dynamique de Bubble. Les autres extraits de code ne fonctionneront pas.

N'oubliez pas de remplacer 'demo' par votre identifiant de locataire FastComments après l'inscription. Si vous êtes connecté à FastComments.com, il devrait déjà être remplacé.

---