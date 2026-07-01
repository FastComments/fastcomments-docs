## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| value | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetSearchSitesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchSitesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getSearchSites 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchSites() {
  const value: string = "customer support"
  const tenantId: string = "tenant-9876"
  const sso: string = "sso-abc123"

  const sites: GetSearchSitesResponse = await getSearchSites(value, tenantId, sso)
  const sitesOnlyTenant: GetSearchSitesResponse = await getSearchSites(undefined, tenantId)
}
[inline-code-end]