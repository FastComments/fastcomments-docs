---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|----------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostSetCommentSpamStatusOptions | No |  |

## Antwort

Rückgabe: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'postSetCommentSpamStatus Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let defaultOpts = PostSetCommentSpamStatusOptions()
let (maybeResp, httpResp) = client.postSetCommentSpamStatus(tenantId = "my-tenant-123", commentId = "cmt-456789", options = defaultOpts)
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]

---