## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createTenantUserBody | CreateTenantUserBody | Ne |  |

## Odgovor

Vraća: [`Option[CreateTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer createTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenantUser(tenantId = "my-tenant-123",
  createTenantUserBody = CreateTenantUserBody(userId = "user-456",
    email = "jane.doe@example.com",
    displayName = "Jane Doe",
    roles = @["editor"],
    isAdmin = false))
if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]

---