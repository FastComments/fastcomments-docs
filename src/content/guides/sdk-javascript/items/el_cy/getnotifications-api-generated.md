## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| userId | string | Όχι |  |
| urlId | string | Όχι |  |
| fromCommentId | string | Όχι |  |
| viewed | boolean | Όχι |  |
| type | string | Όχι |  |
| skip | number | Όχι |  |

## Response

Επιστρέφει: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_98b3f';
const userId: string = 'user_8a3f';
const urlId: string = '/blog/2026/new-feature';
const viewed: boolean = false;
const type: string = 'reply';
const skip: number = 10;
const notifications: GetNotificationsResponse = await getNotifications(tenantId, userId, urlId, undefined, viewed, type, skip);
[inline-code-end]

---