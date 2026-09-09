Webhooks può anche essere gestito tramite l'API REST. Questo è il modo in cui integrazioni come Zapier si iscrivono
agli eventi dei commenti senza toccare la dashboard, e segue il modello REST Hooks: subscribe,
receive events, unsubscribe.

Le iscrizioni API convivono con i webhook configurati nella dashboard. Un evento di commento viene consegnato
al webhook della dashboard per il suo dominio e a ogni iscrizione API che corrisponde, ciascuna come sua
propria consegna. Non c'è limite a un solo subscriber per evento.

## Autenticazione

Ogni richiesta necessita della tua API Key nell'header `x-api-key` (o del parametro di query `API_KEY`) e
del tuo tenant ID nel parametro di query `tenantId`. Entrambi sono mostrati nella pagina API Secret nella dashboard.

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
|-------|--------------|-------------|
| `url` | Sì | Un URL assoluto http o https. |
| `event` | Sì | `comment-created`, `comment-updated` o `comment-deleted`. |
| `domain` | No | Un dominio dalla configurazione del tuo account. Il valore predefinito è `*`, che riceve eventi per tutti i domini. |
| `method` | No | `POST` (predefinito), `PUT` o `DELETE`. |

La risposta contiene l'iscrizione:

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

Iscrivere nuovamente la stessa URL allo stesso evento e dominio restituisce l'iscrizione esistente invece
di crearne una duplicata, così il client può ritentare in sicurezza. Ogni tenant può avere fino a 50 iscrizioni API.

## Elenco

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Restituisce tutti i webhook per il tenant, inclusi quelli gestiti nella dashboard (`"source": "dashboard"`).
Filtra con `event`, `domain` o `source`.

## Annullamento iscrizione

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Eliminare un'iscrizione scarta anche eventuali eventi ancora in coda per essa. Solo le iscrizioni create
tramite l'API possono essere eliminate in questo modo. I webhook della dashboard sono modificati nella pagina Webhooks.

## Payload e firma

Le consegne usano lo stesso payload dei webhook della dashboard (vedi Strutture Dati) e sono firmate con lo stesso
schema HMAC (vedi Sicurezza & Token API). Le iscrizioni API non ricevono mai l'header legacy `token`, quindi
verifica l'header `X-FastComments-Signature` invece.

## Risposta con 410 Gone

Se l'endpoint di un'iscrizione API risponde con HTTP `410 Gone`, FastComments lo interpreta come
un annullamento iscrizione: l'iscrizione viene eliminata insieme ai suoi eventi in coda, e non vengono
tentate ulteriori consegne. I webhook configurati nella dashboard non vengono mai eliminati automaticamente; per loro un 410 è un fallimento ordinario. Qualsiasi altro stato di errore viene ritentato e alla fine disabilita il webhook, come descritto in Come funziona & Gestione dei retry.

## Dashboard

Le iscrizioni API sono elencate nella pagina Webhooks sotto il dominio per cui sono state create, dove un
amministratore può disabilitarle, riabilitarle o eliminarle.