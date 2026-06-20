## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlIdWS | string | Nie |  |
| userIds | string | Nie |  |

## Response

Zwraca: [`Option[GetUserPresenceStatusesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_presence_statuses_response.nim)

## Example

[inline-code-attrs-start title = 'Przykład getUserPresenceStatuses'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserPresenceStatuses(tenantId = "my-tenant-123", urlIdWS = "news/article-title", userIds = "user-123,user-456")
if response.isSome:
  let presenceStatuses = response.get()
  echo presenceStatuses
else:
  echo "No presence data"
[inline-code-end]