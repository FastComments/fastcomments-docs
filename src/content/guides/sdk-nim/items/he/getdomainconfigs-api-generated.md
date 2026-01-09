## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |

## תגובה

מחזיר: [`Option[GetDomainConfigs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getDomainConfigs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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