## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |

## レスポンス

戻り値: [`Option[GetDomainConfigs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs200response.nim)

## 例

[inline-code-attrs-start title = 'getDomainConfigs の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfigs(tenantId = "my-tenant-123")
if response.isSome:
  let domainConfigs = response.get()
  echo "Domain configs received:"
  echo domainConfigs
else:
  echo "Failed to retrieve domain configs."
  echo httpResponse
[inline-code-end]

---