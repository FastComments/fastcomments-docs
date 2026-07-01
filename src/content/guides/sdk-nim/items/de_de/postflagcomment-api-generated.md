## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| options | PostFlagCommentOptions | Nein |  |

## Antwort

Rückgabe: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'postFlagComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = PostFlagCommentOptions()
let (response, httpResponse) = client.postFlagComment(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  options = opts,
)
if response.isSome:
  let result = response.get()
[inline-code-end]