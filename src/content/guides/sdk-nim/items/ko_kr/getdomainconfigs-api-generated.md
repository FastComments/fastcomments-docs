## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## 응답

반환: [`Option[GetDomainConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs_response.nim)

## 예시

[inline-code-attrs-start title = 'getDomainConfigs 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (domainConfigsOpt, httpResponse) = client.getDomainConfigs(tenantId = "my-tenant-123")
if domainConfigsOpt.isSome:
  let domainConfigs = domainConfigsOpt.get()
  echo domainConfigs
[inline-code-end]