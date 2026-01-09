FastComments understøtter også Page Reacts (også kaldet den flydende Like-knap) widgeten til Zyro.

Du kan se den i aktion nederst til højre på denne side!

1. Først, hent koden:

[inline-code-attrs-start title = 'Zyro Flydende Likes-kode'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Åbn derefter sitebyggeren i Zyro.
3. Gå til Webstedsindstillinger i nederste venstre hjørne.
4. Vælg Integrationer.
5. Tilføj den nye kode til *enden* af feltet `Custom code`, og publicer dit site.
6. Du vil ikke se widget'en i forhåndsvisningstilstand, men den vil fremgå i den publicerede version af sitet.

---