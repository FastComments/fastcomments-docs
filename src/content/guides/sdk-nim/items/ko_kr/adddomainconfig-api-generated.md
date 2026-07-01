## 매개변수

| 이름 | 유형 | 필요 여부 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| addDomainConfigParams | AddDomainConfigParams | 아니오 |  |

## 응답

반환: [`Option[AddDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config_response.nim)

## 예시

[inline-code-attrs-start title = 'addDomainConfig 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.addDomainConfig(
  tenantId = "my-tenant-123",
  addDomainConfigParams = default(AddDomainConfigParams)
)

if respOpt.isSome:
  let cfg = respOpt.get()
[inline-code-end]