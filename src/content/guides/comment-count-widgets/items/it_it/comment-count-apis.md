Ci sono alcuni endpoint per ottenere i conteggi, a seconda di cosa vuoi e se vuoi ottenerli da un browser, server o usando l'SDK API.

## Conteggi Pubblici dei Commenti

Puoi ottenere i conteggi pubblici dei commenti usando i widget sopra o usando le API che utilizzano. Queste API sono rimaste invariate dal 2019 e non cambieranno mai.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Questo restituira una struttura come:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

La proprieta `postfix` e sempre inclusa.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Questo restituira una struttura come:

[inline-code-attrs-start title = 'Bulk Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

L'oggetto `counts` viene popolato solo per le pagine che hanno conteggi. La mappa `translations` e sempre presente poiche viene utilizzata per il widget.

### Comportamento degli Endpoint Pubblici / Caching

Gli endpoint pubblici hanno un meccanismo di caching di 60 secondi per gestire i picchi di traffico. Internamente questo e una cache LRU per thread in memoria sul server, quindi potresti vedere i conteggi cambiare leggermente (salire e poi scendere temporaneamente) quando le persone lasciano molti commenti.

Gli endpoint pubblici restituiscono sempre il conteggio *totale* dei commenti, non il conteggio dei commenti radice.

### API Lato Server / SDK

Il modo per ottenere commenti dal tuo server e chiamare l'[API Pages](/guide-api.html#page-structure) e ottenere l'oggetto pagina, che contiene il conteggio totale dei commenti e il conteggio dei commenti radice. Forniamo SDK che ti permettono di chiamare questa API senza costruire manualmente la richiesta API e forniscono valori di ritorno tipizzati.
