## Ajouter un widget de commentaires en direct à vos articles Notion sur Super.so

En plus de Collab Chat, vous pouvez ajouter un widget de commentaires traditionnel au bas de vos articles Notion. Cela permet aux lecteurs de laisser des commentaires et de discuter de l'article dans son ensemble.

### Étapes d'installation

Copiez le code suivant et collez-le dans la section `Body` des paramètres de votre site Super.so :

[inline-code-attrs-start title = 'Widget de commentaires en direct FastComments pour Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Nettoyer l'instance existante
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Créer une nouvelle cible
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Initialiser FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Mettre à jour currentPathname
            currentPathname = window.location.pathname;
        }

        // Chargement initial
        load();

        // Vérifier tous les 500 ms pour des changements
        setInterval(() => {
            // Recharger si pathname a changé
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
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Notes importantes

- Le widget de commentaires apparaîtra au bas de vos articles Notion
- Chaque page obtient son propre fil de commentaires unique basé sur le chemin de l'URL
- Assurez-vous de remplacer `"demo"` par votre ID de locataire réel provenant de votre compte FastComments
- Le widget gère automatiquement le chargement dynamique des pages de Super.so

---