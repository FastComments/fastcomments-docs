## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | No |  |

## Odziv

Vrne: [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## Primer

[inline-code-attrs-start title = 'createQuestionConfig Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let configBody = CreateQuestionConfigBody()
let (maybeResp, httpResp) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = configBody,
)
if maybeResp.isSome:
  let resp = maybeResp.get()
  # uporabite resp po potrebi
[inline-code-end]