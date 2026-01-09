## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| feedPost | FeedPost | 否 |  |

## 回應

回傳: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 範例

[inline-code-attrs-start title = 'updateFeedPost 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let feedPost = FeedPost(
  title: "Local Election Results",
  content: "Updated vote counts across precincts",
  tags: @["politics", "local"],
  authorId: "journalist-32",
  isPublished: true,
  views: 124
)

let (response, httpResponse) = client.updateFeedPost(tenantId = "my-tenant-123", id = "post-456", feedPost = feedPost)

if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---