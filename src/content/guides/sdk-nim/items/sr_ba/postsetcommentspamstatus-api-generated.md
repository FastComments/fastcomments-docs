## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| options | PostSetCommentSpamStatusOptions | Ne |  |

## Odgovor

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'postSetCommentSpamStatus Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let defaultOpts = PostSetCommentSpamStatusOptions()
let (maybeResp, httpResp) = client.postSetCommentSpamStatus(tenantId = "my-tenant-123", commentId = "cmt-456789", options = defaultOpts)
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]