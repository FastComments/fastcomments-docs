Ora possiamo copiare lo snippet di codice seguente. Usa il pulsante di copia che appare in alto a destra nello snippet.

Ci sono alcune cose che puoi configurare nel codice, vedi le righe 4-7.

[inline-code-attrs-start title = 'Codice per singola pagina Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // il tuo ID account
    }];
</script>
[inline-code-end]

Dovrebbe apparire così:

<div class="screenshot white-bg">
    <div class="title">Incolla e Salva</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Incolla e Salva" />
</div>

Ora clicca su Salva in alto a destra.

Nota che l'opzione `Preview in Safe Mode` non funzionerà, ma il widget apparirà quando visiterai il tuo sito.

Se riscontri problemi, assicurati che nella parte inferiore non ci sia scritto `"tenantId": "demo"`. Dovrebbe mostrare il tuo tenant id se sei connesso. In caso contrario, contatta il supporto.