## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postId | string | Ne |  |
| reactBodyParams | ReactBodyParams | Ne |  |
| isUndo | bool | Ne |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[ReactFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer reactFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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