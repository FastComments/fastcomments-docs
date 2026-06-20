## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| postId | string | いいえ |  |
| reactBodyParams | ReactBodyParams | いいえ |  |
| isUndo | bool | いいえ |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返り値: [`Option[ReactFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_response.nim)

## 例

[inline-code-attrs-start title = 'reactFeedPostPublic の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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