## 參數

| Name | Type | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createFeedPostParams | CreateFeedPostParams | 否 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[CreateFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_public200response.nim)

## 範例

[inline-code-attrs-start title = 'createFeedPostPublic 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createFeedPostPublic(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(
    title = "Product Launch Announcement",
    content = "We just launched a new commenting feature to improve engagement.",
    authorId = "team-product",
    url = "news/product-launch",
    tags = @["launch", "comments"],
    isFeatured = false
  ),
  broadcastId = "broadcast-009",
  sso = ""
)
if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]