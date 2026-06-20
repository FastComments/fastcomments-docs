## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| skip | int | Não |  |

## Resposta

Retorna: [`Option[GetSSOUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getSSOUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if response.isSome:
  let ssoUsers = response.get()
  echo ssoUsers
else:
  echo "No SSO users returned; HTTP response:", httpResponse
[inline-code-end]

---