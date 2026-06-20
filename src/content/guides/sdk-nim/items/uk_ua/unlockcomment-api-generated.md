## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад unLockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let commentId = "cmt-987654321"
let (response, httpResponse) = client.unLockComment(
  tenantId = tenantId,
  commentId = commentId,
  broadcastId = "",
  sso = ""
)
if response.isSome:
  let apiResp = response.get()
  echo "Unlocked comment ", commentId, " for tenant ", tenantId
else:
  echo "Unlock failed, HTTP status: ", $httpResponse.status
[inline-code-end]