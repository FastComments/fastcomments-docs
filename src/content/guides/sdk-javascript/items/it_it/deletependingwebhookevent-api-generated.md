## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4e2b';
const pendingEventId: string = '9f7b6a8c-3b2a-4c0d-a8e5-1234567890ab';
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, pendingEventId);
console.log(result);
[inline-code-end]

---