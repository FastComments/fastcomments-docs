## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 否 |  |
| options | DeleteFeedPostPublicOptions | 否 |  |

## 回應

返回: [`Option[DeleteFeedPostPublicResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public_response.nim)

## 範例

[inline-code-attrs-start title = 'deleteFeedPostPublic 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/article-42",
  options = DeleteFeedPostPublicOptions()
)

if response.isSome:
  let deleteResp = response.get()
[inline-code-end]