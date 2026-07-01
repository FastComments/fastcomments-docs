## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| options | GetSearchSitesOptions | Non |  |

## Réponse

Retourne : [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getSearchSites'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchResult, httpResp) = client.getSearchSites(tenantId = "my-tenant-123", options = GetSearchSitesOptions())
if searchResult.isSome:
  let siteResp = searchResult.get()
[inline-code-end]

---