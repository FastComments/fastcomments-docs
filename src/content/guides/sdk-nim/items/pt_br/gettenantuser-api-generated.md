## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[GetTenantUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_user200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUser(tenantId = "my-tenant-123", id = "user-456")
if response.isSome:
  let tenantUser = response.get()
  discard tenantUser
else:
  discard httpResponse
[inline-code-end]

---