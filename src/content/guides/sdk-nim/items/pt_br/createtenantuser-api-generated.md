## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| createTenantUserBody | CreateTenantUserBody | Não |  |

## Resposta

Retorna: [`Option[CreateTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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