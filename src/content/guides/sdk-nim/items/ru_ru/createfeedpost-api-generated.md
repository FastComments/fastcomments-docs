## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createFeedPostParams | CreateFeedPostParams | Нет |  |
| broadcastId | string | Нет |  |
| isLive | bool | Нет |  |
| doSpamCheck | bool | Нет |  |
| skipDupCheck | bool | Нет |  |

## Ответ

Возвращает: [`Option[CreateFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_posts_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример createFeedPost'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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