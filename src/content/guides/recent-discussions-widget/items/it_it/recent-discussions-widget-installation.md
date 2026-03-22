Il widget Discussioni recenti mostra le pagine del tuo sito che hanno l'attività di commento più recente. Ogni voce visualizza il titolo della pagina, la data dell'ultima attività e il numero totale di commenti. Rileva automaticamente gli sfondi scuri e adatta di conseguenza il suo stile.

## Installazione base

[inline-code-attrs-start title = 'Installazione del widget Discussioni recenti'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Opzioni di configurazione

La funzione `FastCommentsRecentDiscussionsV2` accetta le seguenti opzioni di configurazione:

- **tenantId** (required): il tuo FastComments tenant ID
- **count** (optional): numero di pagine da mostrare. Il valore predefinito è `20`, massimo `100`
- **hasDarkBackground** (optional): forza lo stile in modalità scura. Viene rilevato automaticamente dallo sfondo della pagina se non impostato

## Esempi avanzati

### Conteggio personalizzato

[inline-code-attrs-start title = 'Discussioni recenti con conteggio personalizzato'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### Forzare la modalità scura

[inline-code-attrs-start title = 'Discussioni recenti in modalità scura'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

---