## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| meta | string | いいえ |  |
| skip | float64 | いいえ |  |

## レスポンス

戻り値: [`Option[GetTenants_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants200response.nim)

## 例

[inline-code-attrs-start title = 'getTenants の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenants(tenantId = "my-tenant-123", meta = "include=details", skip = 0.0)
if response.isSome:
  let tenants = response.get()
  echo "Received tenants: ", repr(tenants)
else:
  echo "Failed to retrieve tenants"
[inline-code-end]

---