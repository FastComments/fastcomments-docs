## Parametri

| Naziv | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createTenantUserBody | CreateTenantUserBody | Ne |  |

## Odgovor

Vraća: [`Option[CreateTenantUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer createTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateTenantUserBody(
  email = "jane.doe@acmepub.com",
  displayName = "Jane Doe",
  disabled = false
)
let (response, httpResponse) = client.createTenantUser(tenantId = "my-tenant-123", createTenantUserBody = createBody)
if response.isSome:
  let createdUser = response.get()
  echo createdUser
[inline-code-end]

---