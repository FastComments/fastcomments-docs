FastComments prend également en charge le widget Page Reacts (alias bouton J’aime flottant) pour Hostinger.

Vous pouvez le voir en action en bas à droite de cette page !

### Remarque !

Ces instructions concernent le Hostinger Site Builder. Si vous utilisez Hostinger *WordPress*, récupérez simplement le code ci-dessous et ajoutez-le à votre site WordPress en utilisant [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), qui est un plugin gratuit et facile pour ajouter de petits extraits de code à votre site.

1. Tout d'abord, récupérez le code :

[inline-code-attrs-start title = 'Code des Likes flottants Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Ensuite, dans Hostinger, ouvrez le site builder.
3. Allez dans Website Settings en bas à gauche.
4. Sélectionnez Integrations.
5. Ajoutez le nouveau code à la *fin* du champ `Custom code`, puis publiez votre site.
6. Vous ne verrez pas le widget en mode aperçu, mais il apparaîtra sur la version publiée du site.