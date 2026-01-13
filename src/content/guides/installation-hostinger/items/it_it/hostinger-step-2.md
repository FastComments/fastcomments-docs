Aggiungiamo ora il codice del nostro widget.

Copia il codice qui sotto. Assicurati di aver eseguito l'accesso a [fastcomments.com](https://fastcomments.com)
e ricarica questa pagina se non l'hai fatto, in modo che il codice venga precompilato con le informazioni del tuo account; altrimenti verrà mostrato il codice demo.

Ora copiamo il codice:

[inline-code-attrs-start title = 'Codice Commenti Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Torniamo al costruttore del sito e clicchiamo `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Inserisci codice</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Inserisci codice" />
</div>

### Nota!

È importante utilizzare il codice sopra e non gli snippet di altre documentazioni, poiché questo frammento è stato creato appositamente
per Hostinger.

Ora dovresti avere qualcosa di simile al seguente, che appare vuoto. Questo è normale. Muovi il cursore del mouse sull'area
dove dovrebbe essere il widget:

<div class="screenshot white-bg">
    <div class="title">Widget aggiunto</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget aggiunto" />
</div>

Ora trascina il widget per impostare la dimensione desiderata; lo vedrai apparire:

<div class="screenshot white-bg">
    <div class="title">Ridimensionalo</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Ridimensionalo" />
</div>

...e ora visualizza l'anteprima e salva!