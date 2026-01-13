Dans l'étape suivante, vous devez copier le code du widget préfabriqué ci-dessous.

Tant que vous êtes connecté à FastComments.com, l'extrait de code ci-dessous contiendra déjà les informations de votre compte. Copions-le :

[inline-code-attrs-start title = 'Code Super.so FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Nettoyer l'instance existante
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Supprimer la barre supérieure existante si elle existe
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Créer une nouvelle barre supérieure
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Initialiser FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Mettre à jour le pathname actuel
            currentPathname = window.location.pathname;
        }

        // Chargement initial
        load();

        // Vérifier les changements toutes les 500 ms
        setInterval(() => {
            // Recharger si le pathname a changé
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Recharger si le widget a été supprimé
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Recharger si le conteneur a été vidé
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Collez maintenant dans la zone `Body` :

<div class="screenshot white-bg">
    <div class="title">Code collé</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Code collé" />
</div>

Si vous voyez un "ceci est un message de démonstration" après avoir collé le code :

- Assurez-vous d'être connecté à votre compte fastcomments.com.
- Assurez-vous que les cookies tiers sont activés.
- Ensuite, actualisez cette page et copiez à nouveau l'extrait de code. Il devrait avoir `tenantId` rempli avec l'identifiant de votre tenant.