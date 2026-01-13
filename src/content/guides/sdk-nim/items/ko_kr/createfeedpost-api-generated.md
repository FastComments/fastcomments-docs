## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createFeedPostParams | CreateFeedPostParams | 아니오 |  |
| broadcastId | string | 아니오 |  |
| isLive | bool | 아니오 |  |
| doSpamCheck | bool | 아니오 |  |
| skipDupCheck | bool | 아니오 |  |

## 응답

반환: [`Option[CreateFeedPost_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post200response.nim)

## 예제

[inline-code-attrs-start title = 'createFeedPost 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createParams = CreateFeedPostParams(
  content = "We're rolling out realtime comments to all users!",
  title = "Realtime Comments Rollout",
  url = "news/realtime-comments-rollout-2025",
  authorId = "prod-team",
  tags = @["release", "comments"]
)

let (response, httpResponse) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = createParams,
  broadcastId = "broadcast-2025-11",
  isLive = true,
  doSpamCheck = true,
  skipDupCheck = false
)

if response.isSome:
  let created = response.get()
  echo "Feed post created, id: ", $created.id
else:
  echo "Failed to create feed post, HTTP status: ", $httpResponse.statusCode
[inline-code-end]

---