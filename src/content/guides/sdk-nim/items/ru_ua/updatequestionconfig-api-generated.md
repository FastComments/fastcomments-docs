## Параметры

| Name | Type | Обязательно | Описание |
|------|------|------------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример updateQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateQuestionConfig(tenantId = "my-tenant-123", id = "question-config-456", updateQuestionConfigBody = default(UpdateQuestionConfigBody))
if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---