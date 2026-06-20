## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| meta | string | No |  |
| skip | float64 | No |  |

## Risposta

Restituisce: [`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTenants'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenants(tenantId = "my-tenant-123", meta = "env=production", skip = 0.0)
if response.isSome:
  let tenantsResp = response.get()
  discard tenantsResp
  echo "Tenants fetched successfully"
else:
  echo "Request failed with status ", httpResponse.status
[inline-code-end]

---