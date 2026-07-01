---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |
| options | DeleteTenantUserOptions | No |  |

## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'deleteTenantUser Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  options = DeleteTenantUserOptions(),
)
if response.isSome:
  let empty = response.get()
  echo "User successfully deleted"
[inline-code-end]

---