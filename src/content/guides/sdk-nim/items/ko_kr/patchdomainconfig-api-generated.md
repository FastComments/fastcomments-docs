## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| domainToUpdate | string | 아니오 |  |
| patchDomainConfigParams | PatchDomainConfigParams | 아니오 |  |

## 응답

반환: [`Option[PatchDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_domain_config_response.nim)

## 예시

[inline-code-attrs-start title = 'patchDomainConfig 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.patchDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news.mywebsite.com",
  patchDomainConfigParams = PatchDomainConfigParams()
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]

---