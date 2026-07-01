## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlIdWS | string | Ні |  |
| userIds | string | Ні |  |

## Відповідь

Повертає: [`Option[GetUserPresenceStatusesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_presence_statuses_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserPresenceStatuses'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (presenceOpt, httpResp) = client.getUserPresenceStatuses(tenantId = "my-tenant-123", urlIdWS = "news/article-title", userIds = "user42")
if presenceOpt.isSome:
  let presence = presenceOpt.get()
[inline-code-end]