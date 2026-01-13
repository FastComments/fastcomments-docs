## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| skip | float64 | Não |  |

## Resposta

Retorna: [`Option[GetQuestionConfigs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getQuestionConfigs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfigs(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let configs = response.get()
  discard configs
[inline-code-end]

---