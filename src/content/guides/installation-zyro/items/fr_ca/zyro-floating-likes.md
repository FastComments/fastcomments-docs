---
FastComments prend également en charge le widget Page Reacts (aussi appelé bouton J'aime flottant) pour Zyro.

Vous pouvez le voir en action en bas à droite de cette page !

1. D'abord, récupérez le code :

[inline-code-attrs-start title = 'Code des boutons flottants pour Zyro (Page Reacts)'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Ensuite, dans Zyro, ouvrez l'éditeur de site.
3. Allez dans Paramètres du site en bas à gauche.
4. Sélectionnez Intégrations.
5. Ajoutez le nouveau code à la *fin* du champ `Custom code`, puis publiez votre site.
6. Vous ne verrez pas le widget en mode aperçu, mais il apparaîtra sur la version publiée du site.

---