## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| postId | string | 否 |  |
| reactBodyParams | ReactBodyParams | 否 |  |
| isUndo | bool | 否 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[ReactFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_response.nim)

## 示例

[inline-code-attrs-start title = 'reactFeedPostPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/article-2026-06-19",
  reactBodyParams = ReactBodyParams(reactType = "heart", tags = @["breaking", "editorial"]),
  isUndo = false,
  broadcastId = "broadcast-789",
  sso = "sso-token-abc123"
)
if response.isSome:
  let react = response.get()
  echo react
else:
  echo "No response from reactFeedPostPublic, HTTP status:", httpResponse.statusCode
[inline-code-end]

---