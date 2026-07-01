## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| options | GetSearchSuggestOptions | Non |  |

## Réponse

Renvoie : [`Option[ModerationSuggestResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_suggest_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getSearchSuggest'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (suggestOpt, httpResp) = client.getSearchSuggest(
  tenantId = "my-tenant-123",
  options = GetSearchSuggestOptions(),
)

if suggestOpt.isSome:
  let suggest = suggestOpt.get()
  echo suggest
[inline-code-end]