## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nej |  |

## Svar

Returnerer: [`Option[GetTenant_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant200response.nim)

## Eksempel

[inline-code-attrs-start title = 'getTenant Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenant(tenantId = "my-tenant-123", id = "")
if response.isSome:
  let tenant = response.get()
  echo "Tenant retrieved"
  discard tenant
else:
  echo "No tenant found"
  echo "HTTP status:", httpResponse.status
[inline-code-end]

---