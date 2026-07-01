## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| createTenantUserBody | CreateTenantUserBody | Non |  |

## Réponse

Retourne : [`Option[CreateTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple createTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userBody = CreateTenantUserBody(
  email: "john.doe@example.com",
  name: "John Doe",
  password: "s3cr3tP@ss",
  role: "admin"
)
let (optResp, httpResp) = client.createTenantUser(
  tenantId = tenantId,
  createTenantUserBody = userBody
)
if optResp.isSome:
  let resp = optResp.get()
  echo resp.userId
[inline-code-end]