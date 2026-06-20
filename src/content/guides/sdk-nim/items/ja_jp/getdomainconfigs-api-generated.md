---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |

## レスポンス

戻り値: [`Option[GetDomainConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs_response.nim)

## 例

[inline-code-attrs-start title = 'getDomainConfigsの例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfigs(tenantId = "my-tenant-123")
if response.isSome:
  let domainConfigs = response.get()
  echo "Received domain configs for tenant my-tenant-123"
  echo domainConfigs
else:
  echo "No domain configs returned"
[inline-code-end]

---