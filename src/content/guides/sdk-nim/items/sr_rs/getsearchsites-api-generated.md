## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| options | GetSearchSitesOptions | No |  |

## Odgovor

Returns: [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getSearchSites'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchResult, httpResp) = client.getSearchSites(tenantId = "my-tenant-123", options = GetSearchSitesOptions())
if searchResult.isSome:
  let siteResp = searchResult.get()
[inline-code-end]