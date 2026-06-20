---

req
tenantId
afterId

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| afterId | string | 아니요 |  |
| limit | int | 아니요 |  |
| tags | seq[string] | 아니요 |  |
| sso | string | 아니요 |  |
| isCrawler | bool | 아니요 |  |
| includeUserInfo | bool | 아니요 |  |

## 응답

반환: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## 예제

[inline-code-attrs-start title = 'getFeedPostsPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsPublic(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[],
  sso = "",
  isCrawler = false,
  includeUserInfo = false
)
if response.isSome:
  let feed = response.get()
  discard feed
[inline-code-end]

---