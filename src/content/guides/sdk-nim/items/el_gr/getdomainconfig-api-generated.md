## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| domain | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let domainConfig = response.get()
  echo "Loaded domain config for tenant my-tenant-123:", $domainConfig
else:
  echo "No domain config; HTTP status:", $httpResponse.status
[inline-code-end]

---