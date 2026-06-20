---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| feedPost | FeedPost | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'updateFeedPost 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let feedPost: FeedPost = FeedPost(title: "City Council Approves New Waterfront Park",
  content: "The council voted 5-2 to approve funding and a timeline for construction.",
  tags: @["local", "parks", "city"],
  published: true)

let (response, httpResponse) = client.updateFeedPost(tenantId = "my-tenant-123", id = "post-456", feedPost = feedPost)

if response.isSome:
  let apiResp = response.get()
  echo "Feed post updated successfully"
  echo apiResp
else:
  echo "Failed to update feed post"
  echo httpResponse
[inline-code-end]

---