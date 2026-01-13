## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domain | string | Nein |  |

## Antwort

Gibt zurück: [`Option[DeleteDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config200response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für deleteDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let result = response.get()
  echo "Deleted domain config result: ", result
else:
  echo "No response body, HTTP status: ", $httpResponse.status
[inline-code-end]

---