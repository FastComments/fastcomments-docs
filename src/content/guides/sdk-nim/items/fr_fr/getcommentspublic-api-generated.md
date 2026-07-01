req
tenantId
urlId

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| options | GetCommentsPublicOptions | Non |  |

## Réponse

Renvoie : [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---