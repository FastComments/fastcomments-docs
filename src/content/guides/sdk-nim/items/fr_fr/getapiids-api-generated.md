## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetApiIdsOptions | No |  |

## Réponse

Renvoie : [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getApiIds'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetApiIdsOptions()
let (maybeResponse, httpResponse) = client.getApiIds(tenantId = "my-tenant-123", options = opts)
if maybeResponse.isSome:
  let response = maybeResponse.get()
  echo response
[inline-code-end]