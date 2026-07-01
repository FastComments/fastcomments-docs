## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| options | PostSetCommentSpamStatusOptions | Nee |  |

## Respons

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postSetCommentSpamStatus Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let defaultOpts = PostSetCommentSpamStatusOptions()
let (maybeResp, httpResp) = client.postSetCommentSpamStatus(tenantId = "my-tenant-123", commentId = "cmt-456789", options = defaultOpts)
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]

---