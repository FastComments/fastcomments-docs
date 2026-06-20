## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| domainToUpdate | string | 否 |  |
| patchDomainConfigParams | PatchDomainConfigParams | 否 |  |

## 回應

回傳: [`Option[PatchDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_domain_config_response.nim)

## 範例

[inline-code-attrs-start title = 'patchDomainConfig 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let patchParams: PatchDomainConfigParams = PatchDomainConfigParams(
  allowedOrigins = @["https://news.example.com", "https://cdn.news.com"],
  enableComments = true,
  moderationRequired = false,
  maxCommentLength = 2000,
  primaryDomain = "comments.news-site.com"
)
let (response, httpResponse) = client.patchDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "comments.news-site.com",
  patchDomainConfigParams = patchParams
)
if response.isSome:
  let cfg = response.get()
  echo "Patched domain config received:", cfg
else:
  echo "No response body, HTTP status:", httpResponse.statusCode
[inline-code-end]

---