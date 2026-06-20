## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domain | string | Nej |  |

## Svar

Returnerer: [`Option[DeleteDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config_response.nim)

## Eksempel

[inline-code-attrs-start title = 'deleteDomainConfig Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let deleted = response.get()
  echo "DeleteDomainConfig succeeded for tenant ", "my-tenant-123"
else:
  echo "DeleteDomainConfig failed. HTTP status: ", $httpResponse.status
[inline-code-end]

---