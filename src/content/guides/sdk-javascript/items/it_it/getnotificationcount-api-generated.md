## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |

## Risposta

Restituisce: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_abc123';
const userId: string = 'user_987654321';
const urlId: string = 'https://example.com/news/2026/new-features';
const viewed: boolean = false;
const type: string = 'reply';
const notificationCountResponse: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, undefined, viewed, type);
[inline-code-end]

---