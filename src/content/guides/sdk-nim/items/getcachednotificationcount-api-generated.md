## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[GetCachedNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count200response.nim)

## Example

[inline-code-attrs-start title = 'getCachedNotificationCount Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "notif-789")
if response.isSome:
  let cachedCount = response.get()
  echo "Cached notification count response received"
else:
  echo "No cached notification count returned"
[inline-code-end]
