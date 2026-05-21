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
const tenantId: string = "acme-corp";
const userId: string = "user-1024";
const tenantOverride: string | undefined = undefined; // optional override
const effectiveTenantId: string = tenantOverride ?? tenantId;
const notificationCount: GetCachedNotificationCount200Response = await getCachedNotificationCount(effectiveTenantId, userId);
[inline-code-end]
