---
FastComments ondersteunt ook de Page Reacts (ook wel Floating Like-knop genoemd) widget voor Zyro.

Je kunt het in actie zien rechtsonder op deze pagina!

1. Haal eerst de code op:

[inline-code-attrs-start title = 'Zyro zwevende Likes-code'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Open vervolgens in Zyro de sitebuilder.
3. Ga naar Website-instellingen linksonder.
4. Selecteer Integraties.
5. Voeg de nieuwe code toe aan het *einde* van het `Custom code` veld, en publiceer je site.
6. Je zult de widget niet zien in de voorbeeldweergave, maar deze zal verschijnen in de gepubliceerde versie van de site.

---