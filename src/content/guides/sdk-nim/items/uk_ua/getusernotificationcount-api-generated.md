## Параметри

| Name | Type | Required | Опис |
|------|------|----------|------|
| tenantId | string | Так |  |
| sso | string = "" | Ні |  |

## Відповідь

Повертає: [`Option[GetUserNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count_response.nim)

## Приклад

[inline-code-attrs-start title = 'getUserNotificationCount Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]

---