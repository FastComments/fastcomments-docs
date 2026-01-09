## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| addDomainConfigParams | AddDomainConfigParams | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[AddDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.addDomainConfig(tenantId = "my-tenant-123", addDomainConfigParams = AddDomainConfigParams())
if response.isSome:
  let domainConfig = response.get()
  discard domainConfig
else:
  discard httpResponse
[inline-code-end]

---