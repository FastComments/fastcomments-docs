## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| domainToUpdate | string | לא |  |
| patchDomainConfigParams | PatchDomainConfigParams | לא |  |

## תגובה

מחזיר: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-patchDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news/article-crowdsourcing",
  patchDomainConfigParams = PatchDomainConfigParams(
    allowedOrigins = @["https://www.news-site.com"],
    moderated = true,
    maxCommentLength = 1000
  )
)

if response.isSome:
  let domainConfig = response.get()
  echo "Updated domain config received"
else:
  echo "No domain config returned"
[inline-code-end]

---