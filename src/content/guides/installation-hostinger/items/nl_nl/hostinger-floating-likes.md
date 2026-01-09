FastComments ondersteunt ook de Page Reacts (ook wel zwevende Like-knop) widget voor Hostinger.

Je kunt het in actie zien rechtsonder op deze pagina!

### Let op!

Deze instructies gelden voor de Hostinger Site Builder. Als je Hostinger *WordPress* gebruikt, kopieer dan gewoon de onderstaande code en voeg deze toe aan je WordPress-site
met [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), dat is een gratis en gebruiksvriendelijke plugin om kleine codefragmenten aan je site toe te voegen.

1. Pak eerst de code:

[inline-code-attrs-start title = 'Hostinger zwevende vind-ik-leuk-knop code'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Open vervolgens de Site Builder in Hostinger.
3. Ga naar Website-instellingen linksonder.
4. Selecteer Integraties.
5. Voeg de nieuwe code toe aan het *einde* van het veld `Custom code`, en publiceer je site.
6. Je ziet de widget niet in de voorbeeldmodus, maar deze verschijnt in de gepubliceerde versie van de site.

---