## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| value | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetSearchSitesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchSitesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getSearchSites'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchSites() {
  const value: string = "customer support"
  const tenantId: string = "tenant-9876"
  const sso: string = "sso-abc123"

  const sites: GetSearchSitesResponse = await getSearchSites(value, tenantId, sso)
  const sitesOnlyTenant: GetSearchSitesResponse = await getSearchSites(undefined, tenantId)
}
[inline-code-end]