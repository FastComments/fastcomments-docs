## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| urlId | string | Non |  |
| fromCommentId | string | Non |  |
| viewed | boolean | Non |  |
| type | string | Non |  |
| skip | number | Non |  |

## Réponse

Renvoie : [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_98b3f';
const userId: string = 'user_8a3f';
const urlId: string = '/blog/2026/new-feature';
const viewed: boolean = false;
const type: string = 'reply';
const skip: number = 10;
const notifications: GetNotificationsResponse = await getNotifications(tenantId, userId, urlId, undefined, viewed, type, skip);
[inline-code-end]