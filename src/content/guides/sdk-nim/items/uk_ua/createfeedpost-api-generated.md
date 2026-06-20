---
## Параметри

| Name | Type | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createFeedPostParams | CreateFeedPostParams | Ні |  |
| broadcastId | string | Ні |  |
| isLive | bool | Ні |  |
| doSpamCheck | bool | Ні |  |
| skipDupCheck | bool | Ні |  |

## Відповідь

Повертає: [`Option[CreateFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_posts_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад createFeedPost'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let postParams = CreateFeedPostParams(
  title = "Major Acquisition by TechCorp",
  content = "TechCorp has acquired SoftWorks in a deal valued at $1.2B, creating a new market leader.",
  url = "news/tech/major-acquisition",
  tags = @["business", "technology"],
  authorId = "journalist-321"
)

let (response, httpResponse) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = postParams,
  broadcastId = "",
  isLive = false,
  doSpamCheck = false,
  skipDupCheck = false
)

if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]

---