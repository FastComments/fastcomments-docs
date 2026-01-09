## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 否 |  |
| reactBodyParams | ReactBodyParams | 否 |  |
| isUndo | bool | 否 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`Option[ReactFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_public200response.nim)

## 示例

[inline-code-attrs-start title = 'reactFeedPostPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/article-title",
  reactBodyParams = ReactBodyParams(),
  isUndo = false,
  broadcastId = "broadcast-456",
  sso = ""
)
if response.isSome:
  let result = response.get()
  echo "Reaction result: ", result
else:
  echo "Reaction failed, HTTP response: ", httpResponse
[inline-code-end]

---