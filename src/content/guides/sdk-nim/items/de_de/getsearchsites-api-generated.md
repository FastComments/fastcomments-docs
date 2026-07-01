## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| options | GetSearchSitesOptions | Nein |  |

## Antwort

Rückgabe: [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getSearchSites Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchResult, httpResp) = client.getSearchSites(tenantId = "my-tenant-123", options = GetSearchSitesOptions())
if searchResult.isSome:
  let siteResp = searchResult.get()
[inline-code-end]