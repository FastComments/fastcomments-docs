I webhook possono essere gestiti anche tramite la REST API. È così che integrazioni come Zapier si iscrivono agli eventi dei commenti senza toccare la dashboard, e segue il pattern REST Hooks: iscrizione, ricezione degli eventi, annullamento dell'iscrizione.

Le sottoscrizioni API convivono con i webhook configurati nella dashboard. Un evento di commento viene consegnato a ogni webhook che corrisponde al suo dominio, ciascuno come consegna separata, indipendentemente da come il webhook è stato creato.

## Autenticazione

Ogni richiesta richiede la tua API Key nell'header `x-api-key` (o nel parametro query `API_KEY`) e il tuo tenant ID nel parametro query `tenantId`. Entrambi sono mostrati nella pagina API Secret della dashboard.

## Iscrizione

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
|-------|----------|-------------|
| `url` | Sì | Un URL http o https assoluto. |
| `event` | Sì | `comment-created`, `comment-updated` o `comment-deleted`. |
| `domain` | No | Un dominio dalla configurazione del tuo account. Il valore predefinito è `*`, che riceve gli eventi per ogni dominio. |
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

Iscrivere nuovamente lo stesso URL allo stesso evento e dominio restituisce la sottoscrizione esistente invece di crearne un duplicato, quindi un client può riprovare in sicurezza. Ogni tenant può avere fino a 50 sottoscrizioni API.

## Elenco

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Restituisce ogni webhook per il tenant, inclusi quelli gestiti nella dashboard (`"source": "dashboard"`). Filtra con `event`, `domain` o `source`.

## Annullamento iscrizione

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Eliminare una sottoscrizione scarta anche gli eventi ancora in coda per essa. Solo le sottoscrizioni create tramite l'API possono essere eliminate in questo modo; un webhook della dashboard, o un id che non esiste sul tuo account, risponde `404` con codice `not-found`. I webhook della dashboard si modificano nella pagina Webhooks.

## Payload e firma

Le consegne usano lo stesso payload dei webhook della dashboard (vedi Data Structures) e sono firmate con lo stesso schema HMAC (vedi Security & API Tokens). Le sottoscrizioni API non ricevono mai il header legacy `token`, quindi verifica invece l'header `X-FastComments-Signature`.

## Payload di esempio

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Restituisce i commenti più recenti dell'account esattamente nella forma in cui li porta una consegna, così un'integrazione può mostrare dati di esempio reali prima che arrivi il primo evento. `event` è opzionale e viene solo validato, poiché ogni evento consegna lo stesso oggetto commento. `limit` è predefinito a 3 e accetta da 1 a 10. Costa 2 crediti API.

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

## Rispondere con 410 Gone

Se l'endpoint di una sottoscrizione API risponde con HTTP `410 Gone`, FastComments lo tratta come un annullamento dell'iscrizione: la sottoscrizione viene eliminata insieme ai suoi eventi in coda e non vengono tentate ulteriori consegne. I webhook configurati nella dashboard non vengono mai eliminati automaticamente; per loro un 410 è un normale fallimento. Qualsiasi altro stato di fallimento viene ritentato e alla fine disabilita il webhook, come descritto in How it Works & Handling Retries.

## Dashboard

Le sottoscrizioni API appaiono nell'elenco Webhooks con la sorgente **API**, dove un amministratore può modificarle, disabilitarle, riattivarle o eliminarle.