## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| addDomainConfigParams | AddDomainConfigParams | 否 |  |

## 回傳

回傳: [`Option[AddDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config200response.nim)

## 範例

[inline-code-attrs-start title = 'addDomainConfig 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.addDomainConfig(tenantId = "my-tenant-123", addDomainConfigParams = AddDomainConfigParams())
if response.isSome:
  let domainConfig = response.get()
  discard domainConfig
else:
  discard httpResponse
[inline-code-end]

---