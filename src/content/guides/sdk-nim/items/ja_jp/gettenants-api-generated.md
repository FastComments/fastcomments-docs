## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | GetTenantsOptions | いいえ |  |

## レスポンス

戻り値: [`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## 例

[inline-code-attrs-start title = 'getTenants の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getTenants(tenantId = "my-tenant-123", options = GetTenantsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]

---