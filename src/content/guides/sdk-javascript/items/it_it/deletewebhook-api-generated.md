Annullamento iscrizioni (unsubscribe dei REST hook). Solo le sottoscrizioni create tramite questa API possono essere eliminate qui; i webhook gestiti dal dashboard vengono modificati nel dashboard.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const webhookId: string = "wh_98765";

const result: APIEmptyResponse = await deleteWebhook(tenantId, webhookId);
[inline-code-end]