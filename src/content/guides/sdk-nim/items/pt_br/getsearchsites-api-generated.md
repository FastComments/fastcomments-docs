## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| options | GetSearchSitesOptions | Não |  |

## Resposta

Retorna: [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getSearchSites'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchResult, httpResp) = client.getSearchSites(tenantId = "my-tenant-123", options = GetSearchSitesOptions())
if searchResult.isSome:
  let siteResp = searchResult.get()
[inline-code-end]

---