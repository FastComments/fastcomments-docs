## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| postId | string | Ні |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[DeleteFeedPostPublicResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(tenantId = "my-tenant-123", postId = "", broadcastId = "", sso = "")
if response.isSome:
  let deleted = response.get()
  echo "Delete successful"
else:
  echo "Delete failed"
[inline-code-end]

---