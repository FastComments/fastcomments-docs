## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`Option[LockComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_lock_comment200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример unLockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unLockComment(tenantId = "my-tenant-123", commentId = "cmt-7f9a3b2d", broadcastId = "", sso = "")
if response.isSome:
  let lockResult = response.get()
  echo "Unlock response: ", $lockResult
else:
  echo "Unlock failed, HTTP response: ", $httpResponse
[inline-code-end]

---