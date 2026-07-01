## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| options | GetApiIdsOptions | No |  |

## Odgovor

Vraća: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## Primjer

[inline-code-attrs-start title = 'getApiIds Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetApiIdsOptions()
let (maybeResponse, httpResponse) = client.getApiIds(tenantId = "my-tenant-123", options = opts)
if maybeResponse.isSome:
  let response = maybeResponse.get()
  echo response
[inline-code-end]