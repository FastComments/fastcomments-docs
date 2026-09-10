---
I webhook possono anche essere gestiti tramite l'API REST. È così che integrazioni come Zapier si iscrivono agli eventi dei commenti senza toccare la dashboard, e segue il modello REST Hooks: iscrizione, ricezione degli eventi, cancellazione dell'iscrizione.

Le sottoscrizioni API convivono con i webhook configurati nella dashboard. Un evento di commento viene consegnato a ogni webhook che corrisponde al suo dominio, ciascuno come una consegna separata, indipendentemente dal modo in cui il webhook è stato creato.

## Autenticazione

Ogni richiesta richiede la tua API Key nell'intestazione `x-api-key` (o nel parametro di query `API_KEY`) e il tuo ID tenant nel parametro di query `tenantId`. Entrambi sono mostrati nella pagina API Secret nella dashboard.

## Sottoscrizione

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
| `url` | Sì | Un URL assoluto http o https. |
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

Sottoscrivere nuovamente lo stesso URL allo stesso evento e dominio restituisce la sottoscrizione esistente invece di crearne una duplicata, così un client può riprovare in sicurezza. Ogni tenant può avere fino a 50 sottoscrizioni API.

## Elenco

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Restituisce tutti i webhook per il tenant, inclusi quelli gestiti nella dashboard (`"source": "dashboard"`). Filtra con `event`, `domain` o `source`.

## Annulla sottoscrizione

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Eliminare una sottoscrizione elimina anche tutti gli eventi ancora in coda per essa. Solo le sottoscrizioni create tramite l'API possono essere eliminate in questo modo. I webhook della dashboard vengono modificati nella pagina Webhooks.

## Payload e firma

Le consegne utilizzano lo stesso payload dei webhook della dashboard (vedi Strutture dei Dati) e sono firmate con lo stesso schema HMAC (vedi Sicurezza & Token API). Le sottoscrizioni API non ricevono mai l'intestazione legacy `token`, quindi verifica invece l'intestazione `X-FastComments-Signature`.

## Esempi di payload

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Restituisce i commenti più recenti dell'account esattamente nella forma che una consegna trasporta, così un'integrazione può mostrare dati di esempio reali prima che arrivi il primo evento. `event` è opzionale e solo convalidato, poiché ogni evento consegna lo stesso oggetto commento. `limit` ha valore predefinito 3 e accetta da 1 a 10. Costa 2 crediti API.

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

Se l'endpoint di una sottoscrizione API risponde con HTTP `410 Gone`, FastComments lo interpreta come una cancellazione dell'iscrizione: la sottoscrizione viene eliminata insieme ai suoi eventi in coda e non vengono tentate ulteriori consegne. I webhook configurati nella dashboard non vengono mai eliminati automaticamente; per loro un 410 è un errore ordinario. Qualsiasi altro stato di errore viene ritentato e alla fine disabilita il webhook, come descritto in Come funziona & Gestione dei ritardi.

## Dashboard

Le sottoscrizioni API appaiono nell'elenco Webhooks con la fonte **API**, dove un amministratore può modificarle, disabilitarle, riabilitarle o eliminarle.

---