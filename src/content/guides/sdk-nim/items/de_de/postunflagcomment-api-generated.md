## Parameter

| Name     | Type                     | Erforderlich | Beschreibung |
|----------|--------------------------|--------------|--------------|
| tenantId | string                   | Ja           |  |
| commentId| string                   | Ja           |  |
| options  | PostUnFlagCommentOptions | Nein         |  |

## Antwort

Rückgabe: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'postUnFlagComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.postUnFlagComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  options = default(PostUnFlagCommentOptions)
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]