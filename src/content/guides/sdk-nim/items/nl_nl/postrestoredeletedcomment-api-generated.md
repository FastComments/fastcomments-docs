## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| options | PostRestoreDeletedCommentOptions | Nee |  |

## Respons

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postRestoreDeletedComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.postRestoreDeletedComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  options = default(PostRestoreDeletedCommentOptions)
)

if respOpt.isSome:
  let empty = respOpt.get()
  echo "Comment restored"
[inline-code-end]

---