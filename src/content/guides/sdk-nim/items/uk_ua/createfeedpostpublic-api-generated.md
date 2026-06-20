## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createFeedPostParams | CreateFeedPostParams | Ні |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад createFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = CreateFeedPostParams(
  title = "Breaking: Major Update on Product X",
  content = "Today we released Product X v2.0 with performance improvements and bug fixes.",
  author = "jane.doe",
  tags = @["product", "release", "v2"],
  isPinned = false
)

let (response, httpResponse) = client.createFeedPostPublic(tenantId = "my-tenant-123", createFeedPostParams = params, broadcastId = "", sso = "")

if response.isSome:
  let created = response.get()
  echo "Created feed post:", created
else:
  echo "Failed to create feed post; HTTP status:", httpResponse.status
[inline-code-end]