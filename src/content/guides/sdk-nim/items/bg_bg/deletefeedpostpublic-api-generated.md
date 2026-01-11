## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postId | string | Не |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[DeleteFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за deleteFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-456",
  broadcastId = "broadcast-789",
  sso = ""
)
if response.isSome:
  let result = response.get()
[inline-code-end]

---