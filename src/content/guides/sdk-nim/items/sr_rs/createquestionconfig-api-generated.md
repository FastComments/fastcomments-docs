## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | No |  |

## Одговор

Returns: [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## Пример

[inline-code-attrs-start title = 'createQuestionConfig пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let configBody = CreateQuestionConfigBody()
let (maybeResp, httpResp) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = configBody,
)
if maybeResp.isSome:
  let resp = maybeResp.get()
  # користите одговор по потреби
[inline-code-end]