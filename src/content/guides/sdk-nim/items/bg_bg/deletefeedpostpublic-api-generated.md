## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postId | string | Не |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[DeleteFeedPostPublicResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за deleteFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(tenantId = "my-tenant-123", postId = "", broadcastId = "", sso = "")
if response.isSome:
  let deleted = response.get()
  echo "Delete successful"
else:
  echo "Delete failed"
[inline-code-end]

---