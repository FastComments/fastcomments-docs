## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | DeleteTenantUserOptions | No |  |

## Odgovor

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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