Ora possiamo copiare lo snippet di codice seguente. Usa il pulsante copia che appare in alto a destra dello snippet.

Ci sono alcune cose che puoi configurare nel codice, vedi le righe 4-7.

[inline-code-attrs-start title = 'Codice commenti per tutte le pagine di Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // il tuo ID account

        function tryLoad() {
            // prova a caricare per diversi layout
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...poi incolla nell'area del codice e fai clic su Salva. Dovrebbe apparire così, con il codice nel blocco `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Incolla e Salva</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Incolla e Salva" />
</div>

Se riscontri problemi, assicurati che verso il fondo non dica `"tenantId": "demo"`. Dovrebbe mostrare il tuo ID tenant se hai effettuato l'accesso. In caso contrario, contatta il supporto.