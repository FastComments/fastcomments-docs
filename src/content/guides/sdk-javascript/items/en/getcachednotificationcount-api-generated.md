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
const tenantCandidate: string | undefined = "acme-enterprises-9876";
const tenantId: string = tenantCandidate ?? "acme-enterprises-0001";
const notificationId: string = "b6f1a4d2-3c9e-4f8b-9a12-5d3e7c8b9a10";
const cachedCount: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, notificationId);
console.log(cachedCount);
[inline-code-end]
