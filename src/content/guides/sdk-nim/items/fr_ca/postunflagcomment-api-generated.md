## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| options | PostUnFlagCommentOptions | Non |  |

## Réponse

Retourne : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'postUnFlagComment Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.postUnFlagComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  options = default(PostUnFlagCommentOptions)
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]