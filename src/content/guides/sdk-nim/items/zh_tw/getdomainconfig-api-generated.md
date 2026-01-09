## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| domain | string | 否 |  |

## 回應

回傳: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## 範例

[inline-code-attrs-start title = 'getDomainConfig 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let domainConfig = response.get()
  echo "Loaded domain config for tenant my-tenant-123:", $domainConfig
else:
  echo "No domain config; HTTP status:", $httpResponse.status
[inline-code-end]

---