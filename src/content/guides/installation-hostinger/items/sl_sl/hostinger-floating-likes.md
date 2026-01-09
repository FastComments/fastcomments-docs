FastComments podpira tudi gradnik Page Reacts (znan tudi kot lebdeči gumb "Všeč mi je") za Hostinger.

Vidite ga v spodnjem desnem kotu te strani!

### Opomba!

Ta navodila so za Hostinger Site Builder. Če uporabljate Hostinger *WordPress*, potem preprosto vzemite spodnjo kodo in jo dodajte v svojo WordPress stran
z uporabo [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), ki je brezplačen in preprost vtičnik za dodajanje majhnih odsekov kode na vašo stran.

1. Najprej vzemite kodo:

[inline-code-attrs-start title = 'Koda lebdečih všečkov za Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Nato v Hostingerju odprite site builder.
3. Pojdite na Website Settings v spodnjem levem kotu.
4. Izberite Integrations.
5. Dodajte novo kodo na *konec* polja `Custom code` in objavite svojo stran.
6. Gradnika ne boste videli v načinu predogleda, vendar se bo prikazal v objavljeni različici strani.