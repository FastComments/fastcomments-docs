## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## Example

[inline-code-attrs-start title = 'getCachedNotificationCount Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-001';
const notificationId: string = 'user-98765-notif';
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, notificationId);
[inline-code-end]
