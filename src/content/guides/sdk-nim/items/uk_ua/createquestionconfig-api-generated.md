## Параметри

| Назва | Type | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Ні |  |

## Відповідь

Повертає: [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад createQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = CreateQuestionConfigBody(
    label = "Article Question",
    required = true,
    minLength = 20,
    maxLength = 1000,
    allowedTags = @["comment","question","feedback"],
    notifyModerators = false
  )
)
if response.isSome:
  let cfg = response.get()
  echo "Created question config id: ", cfg.id
[inline-code-end]

---