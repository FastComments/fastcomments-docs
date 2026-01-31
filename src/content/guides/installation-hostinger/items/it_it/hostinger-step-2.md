Ora aggiungiamo il codice del nostro widget.

Copia il codice qui sotto. Assicurati di aver effettuato l'accesso a [fastcomments.com](https://fastcomments.com)
e ricarica questa pagina se non lo sei, in modo che il codice venga precompilato con le informazioni del tuo account; altrimenti verrà mostrato il codice demo.

Ora copiamo il codice:

[inline-code-attrs-start title = 'Codice commenti Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Ora torniamo al nostro costruttore del sito e clicchiamo `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Inserisci codice</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Inserisci codice" />
</div>

### Nota!

È importante usare il codice sopra e non gli snippet di codice provenienti da altra documentazione, poiché questo frammento è stato creato specificamente
per Hostinger.

Dovresti ora avere qualcosa di simile a quanto segue, che appare vuoto. Questo è previsto. Muovi il mouse sull'area
dove dovrebbe esserci il widget:

<div class="screenshot white-bg">
    <div class="title">Widget codice aggiunto</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget codice aggiunto" />
</div>

Ora trascina il widget per impostare la dimensione desiderata; vedrai che appare:

<div class="screenshot white-bg">
    <div class="title">Ridimensionalo</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Ridimensionalo" />
</div>

...e ora visualizza l'anteprima e salva!

---