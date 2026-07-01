## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| options | PostUnFlagCommentOptions | Nee |  |

## Respons

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postUnFlagComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.postUnFlagComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  options = default(PostUnFlagCommentOptions)
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]

---