## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| options | GetApiIdsOptions | No |  |

## Risposta

Restituisce: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getApiIds'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetApiIdsOptions()
let (maybeResponse, httpResponse) = client.getApiIds(tenantId = "my-tenant-123", options = opts)
if maybeResponse.isSome:
  let response = maybeResponse.get()
  echo response
[inline-code-end]

---