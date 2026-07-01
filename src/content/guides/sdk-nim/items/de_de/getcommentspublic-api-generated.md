req
tenantId
urlId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|----------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| options | GetCommentsPublicOptions | Nein |  |

## Antwort

Rückgabe: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getCommentsPublic(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetCommentsPublicOptions(
    page = 0,
    pageSize = 10,
    includeDeleted = false,
    tags = @[]
  )
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]