---
Ora copieremo il nostro codice. Se lo snippet di codice riporta `tenantId: "demo"` sulla riga 6 allora dovresti accedere al tuo account FastComments e poi aggiornare questa pagina in modo che lo snippet copiato contenga l'id del tuo account.

[inline-code-attrs-start title = 'Snippet di Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

Ora incollalo nell'editor e fai clic su Salva:

<div class="screenshot white-bg">
    <div class="title">Aggiungi il codice FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Aggiungi il codice FastComments" />
</div>

... poi salva il tuo sito. Ecco fatto!

---