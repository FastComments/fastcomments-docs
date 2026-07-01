## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Nie |  |

## Odpowiedź

Zwraca: [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## Przykład

[inline-code-attrs-start title = 'createQuestionConfig Przykład'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let configBody = CreateQuestionConfigBody()
let (maybeResp, httpResp) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = configBody,
)
if maybeResp.isSome:
  let resp = maybeResp.get()
  # użyj resp w razie potrzeby
[inline-code-end]