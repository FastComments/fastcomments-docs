## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| email | string | Não |  |

## Resposta

Retorna: [`Option[GetSSOUserByEmailAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_email_api_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getSSOUserByEmail'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUserByEmail(tenantId = "my-tenant-123", email = "alice@newsco.com")
if response.isSome:
  let ssoUser = response.get()
  echo "SSO user found: ", ssoUser.email
else:
  echo "No SSO user found. HTTP status: ", httpResponse.status
[inline-code-end]

---