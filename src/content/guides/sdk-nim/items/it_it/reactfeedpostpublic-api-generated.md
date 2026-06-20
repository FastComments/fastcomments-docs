## Parametri

| Name | Type | Obbligatorio | Description |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| postId | string | No |  |
| reactBodyParams | ReactBodyParams | No |  |
| isUndo | bool | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[ReactFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di reactFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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