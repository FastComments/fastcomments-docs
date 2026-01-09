## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| domainToUpdate | string | 아니오 |  |
| updateDomainConfigParams | UpdateDomainConfigParams | 아니오 |  |

## 응답

반환: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## 예제

[inline-code-attrs-start title = 'putDomainConfig 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateParams = UpdateDomainConfigParams(
  allowAnonymous = false,
  moderationEnabled = true,
  allowedOrigins = @["https://news.example.com"],
  maxCommentLength = 2000
)

let (response, httpResponse) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news/example-article",
  updateDomainConfigParams = updateParams
)

if response.isSome:
  let domainCfg = response.get()
  discard domainCfg
[inline-code-end]

---