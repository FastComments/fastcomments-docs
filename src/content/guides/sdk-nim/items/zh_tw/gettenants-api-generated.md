## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| meta | string | 否 |  |
| skip | float64 | 否 |  |

## 回應

回傳：[`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## 範例

[inline-code-attrs-start title = 'getTenants 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenants(tenantId = "my-tenant-123", meta = "env=production", skip = 0.0)
if response.isSome:
  let tenantsResp = response.get()
  discard tenantsResp
  echo "Tenants fetched successfully"
else:
  echo "Request failed with status ", httpResponse.status
[inline-code-end]

---