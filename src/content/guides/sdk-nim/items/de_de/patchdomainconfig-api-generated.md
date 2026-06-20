## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domainToUpdate | string | Nein |  |
| patchDomainConfigParams | PatchDomainConfigParams | Nein |  |

## Antwort

Gibt zurück: [`Option[PatchDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_domain_config_response.nim)

## Beispiel

[inline-code-attrs-start title = 'patchDomainConfig-Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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