## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[GetQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (questionConfigOpt, httpResp) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "question-987")
if questionConfigOpt.isSome:
  let config = questionConfigOpt.get()
  echo config
[inline-code-end]

---