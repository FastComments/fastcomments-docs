## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| options | GetApiCommentsOptions | Non |  |

## Réponse

Retourne : [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getApiComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getApiComments(tenantId = "my-tenant-123", options = GetApiCommentsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  # un traitement supplémentaire peut être effectué avec `resp`
[inline-code-end]

---