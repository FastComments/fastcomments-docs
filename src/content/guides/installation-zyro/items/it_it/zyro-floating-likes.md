FastComments supporta anche il widget Page Reacts (noto anche come pulsante "Mi piace" flottante) per Zyro.

Puoi vederlo in azione in basso a destra di questa pagina!

1. Per prima cosa, copia il codice:

[inline-code-attrs-start title = 'Codice Mi piace flottante Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Poi, in Zyro, apri l'editor del sito.
3. Vai su Impostazioni del sito web in basso a sinistra.
4. Seleziona Integrazioni.
5. Aggiungi il nuovo codice alla *fine* del campo `Custom code`, e pubblica il sito.
6. Non vedrai il widget in modalità anteprima, ma apparirà nella versione pubblicata del sito.