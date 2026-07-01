## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Non |  |

## Réponse

Retourne : [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple createQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let configBody = CreateQuestionConfigBody()
let (maybeResp, httpResp) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = configBody,
)
if maybeResp.isSome:
  let resp = maybeResp.get()
  # use resp as needed
[inline-code-end]