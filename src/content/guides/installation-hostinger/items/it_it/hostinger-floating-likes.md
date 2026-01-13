---
FastComments supporta anche il widget Page Reacts (noto anche come pulsante "Mi piace" flottante) per Hostinger.

Puoi vederlo in azione in basso a destra di questa pagina!

### Nota!

Queste istruzioni sono per l'Hostinger Site Builder. Se stai usando Hostinger *WordPress*, prendi il codice qui sotto e aggiungilo al tuo sito WordPress
usando [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), che è un plugin gratuito e semplice per aggiungere piccoli snippet di codice al tuo sito.

1. Per prima cosa, prendi il codice:

[inline-code-attrs-start title = 'Codice Mi piace flottante per Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Poi, in Hostinger, apri il site builder.
3. Vai a Website Settings in basso a sinistra.
4. Seleziona Integrations.
5. Aggiungi il nuovo codice alla *fine* del campo `Custom code`, e pubblica il tuo sito.
6. Non vedrai il widget in modalità anteprima, ma apparirà nella versione pubblicata del sito.

---