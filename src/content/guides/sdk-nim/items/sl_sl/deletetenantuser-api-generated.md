## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| options | DeleteTenantUserOptions | Ne |  |

## Odgovor

Vrne: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'deleteTenantUser Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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