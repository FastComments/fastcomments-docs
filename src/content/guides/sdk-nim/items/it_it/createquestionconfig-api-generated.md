## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createQuestionConfigBody | CreateQuestionConfigBody | No |  |

## Risposta

Restituisce: [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio createQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let configBody = CreateQuestionConfigBody()
let (maybeResp, httpResp) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = configBody,
)
if maybeResp.isSome:
  let resp = maybeResp.get()
  # usa resp secondo necessità
[inline-code-end]