---
FastComments podpira tudi pripomoček Page Reacts (znan tudi kot lebdeči gumb »Všeč mi je«) za Zyro.

Lahko ga vidite v spodnjem desnem kotu te strani!

1. Najprej pridobite kodo:

[inline-code-attrs-start title = 'Zyro koda za lebdeče všečke'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Nato v Zyro odprite urejevalnik spletnega mesta.
3. Pojdite na Nastavitve spletnega mesta v spodnjem levem kotu.
4. Izberite Integracije.
5. Dodajte novo kodo na *konec* polja `Custom code` in objavite svoje spletno mesto.
6. Widgeta ne boste videli v načinu predogleda, se bo pa prikazal na objavljeni različici spletnega mesta.

---