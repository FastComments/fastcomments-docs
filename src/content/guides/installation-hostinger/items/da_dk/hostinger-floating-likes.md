FastComments understøtter også Page Reacts (også kaldet den flydende Like-knap) widget til Hostinger.

Du kan se den i aktion i nederste højre hjørne på denne side!

### Bemærk!

Disse instruktioner er til Hostinger Site Builder. Hvis du bruger Hostinger *WordPress*, så kopier blot koden nedenfor, og tilføj den til dit WordPress-site
ved hjælp af [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), som er et gratis og nemt plugin til at tilføje små kodeudsnit til dit site.

1. Først skal du hente koden:

[inline-code-attrs-start title = 'Hostinger Flydende Likes-kode'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Åbn derefter Site Builder i Hostinger.
3. Gå til Website Settings i nederste venstre hjørne.
4. Vælg Integrations.
5. Tilføj den nye kode til *enden* af feltet `Custom code`, og udgiv dit site.
6. Du vil ikke se widgeten i forhåndsvisningstilstand, men den vil dukke op på den publicerede version af sitet.