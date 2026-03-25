## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| urlId | string | Non |  |
| fromCommentId | string | Non |  |
| viewed | boolean | Non |  |
| type | string | Non |  |

## Réponse

Retourne : [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_abc123';
const userId: string = 'user_987654321';
const urlId: string = 'https://example.com/news/2026/new-features';
const viewed: boolean = false;
const type: string = 'reply';
const notificationCountResponse: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, undefined, viewed, type);
[inline-code-end]

---