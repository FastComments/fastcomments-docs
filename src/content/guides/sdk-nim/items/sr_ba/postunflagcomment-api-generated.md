## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| options | PostUnFlagCommentOptions | Ne |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'postUnFlagComment Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.postUnFlagComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  options = default(PostUnFlagCommentOptions)
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]