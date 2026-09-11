I Webhook possono anche essere gestiti tramite l'API REST. Questo è il modo in cui integrazioni come Zapier si iscrivono agli eventi dei commenti senza toccare il cruscotto, e segue il modello REST Hooks: subscribe, receive events, unsubscribe.

Le sottoscrizioni API convivono con i webhook configurati nel cruscotto. Un evento di commento viene consegnato a ogni webhook che corrisponde al suo dominio, ciascuno come una consegna separata, indipendentemente dal modo in cui il webhook è stato creato.

## Authentication

Ogni richiesta richiede la tua API Key nell'intestazione `x-api-key` (o nel parametro di query `API_KEY`) e il tuo tenant ID nel parametro di query `tenantId`. Entrambi sono mostrati nella pagina API Secret nel cruscotto.

## Subscribe

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Campo | Obbligatorio | Descrizione |
|-------|--------------|-------------|
| `url` | Sì | Un URL http o https assoluto. |
| `event` | Sì | `comment-created`, `comment-updated` o `comment-deleted`. |
| `domain` | No | Un dominio dalla configurazione del tuo account. Il valore predefinito è `*`, che riceve eventi per tutti i domini. |
| `method` | No | `POST` (predefinito), `PUT` o `DELETE`. |

La risposta contiene la sottoscrizione:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Iscrivere nuovamente lo stesso URL allo stesso evento e dominio restituisce la sottoscrizione esistente invece di crearne una duplicata, così un client può riprovare in sicurezza. Ogni tenant può avere fino a 50 sottoscrizioni API.

## List

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Restituisce tutti i webhook per il tenant, inclusi quelli gestiti nel cruscotto (`"source": "dashboard"`). Filtra con `event`, `domain` o `source`.

## Unsubscribe

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Eliminare una sottoscrizione scarta anche tutti gli eventi ancora in coda per essa. Solo le sottoscrizioni create tramite l'API possono essere eliminate in questo modo; un webhook del cruscotto, o un ID che non esiste nel tuo account, restituisce `404` con il codice `not-found`. I webhook del cruscotto vengono modificati nella pagina Webhooks.

## Payloads and signing

Le consegne utilizzano lo stesso payload dei webhook del cruscotto (vedi Data Structures) e sono firmate con lo stesso schema HMAC (vedi Security & API Tokens). Le sottoscrizioni API non ricevono mai l'intestazione legacy `token`, quindi verifica invece l'intestazione `X-FastComments-Signature`.

## Sample payloads

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Restituisce i commenti più recenti dell'account esattamente nella forma che una consegna trasporta, così un'integrazione può mostrare dati di esempio reali prima che arrivi il primo evento. `event` è opzionale e solo convalidato, poiché ogni evento consegna lo stesso oggetto commento. `limit` è impostato di default a 3 e accetta valori da 1 a 10. Costa 2 crediti API.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Responding with 410 Gone

Se l'endpoint di una sottoscrizione API risponde con HTTP `410 Gone`, FastComments lo interpreta come una cancellazione dell'iscrizione: la sottoscrizione viene eliminata insieme ai suoi eventi in coda, e non vengono tentate ulteriori consegne. I webhook configurati nel cruscotto non vengono mai eliminati automaticamente; per loro un 410 è un errore ordinario. Qualsiasi altro stato di errore viene ritentato e alla fine disabilita il webhook, come descritto in How it Works & Handling Retries.

## Dashboard

Le sottoscrizioni API appaiono nell'elenco Webhooks con la sorgente **API**, dove un amministratore può modificarle, disabilitarle, riabilitarle o eliminarle.

---