## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| tenantId | string | Sim |  |
| options | SetTrustFactorOptions | Não |  |

## Resposta

Retorna: [`Option[SetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_user_trust_factor_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo setTrustFactor'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = SetTrustFactorOptions(userId = "user-456", trustFactor = 5, reason = "spam detection")
let (trustResponse, httpResponse) = client.setTrustFactor(tenantId = "my-tenant-123", options = opts)
if trustResponse.isSome:
  let result = trustResponse.get()
[inline-code-end]

---