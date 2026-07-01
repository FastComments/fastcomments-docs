## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| value | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 回應

回傳：[`GetSearchSitesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchSitesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getSearchSites 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchSites() {
  const value: string = "customer support"
  const tenantId: string = "tenant-9876"
  const sso: string = "sso-abc123"

  const sites: GetSearchSitesResponse = await getSearchSites(value, tenantId, sso)
  const sitesOnlyTenant: GetSearchSitesResponse = await getSearchSites(undefined, tenantId)
}
[inline-code-end]