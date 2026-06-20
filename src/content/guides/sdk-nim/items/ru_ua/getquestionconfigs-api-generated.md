## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| skip | float64 | Нет |  |

## Ответ

Возвращает: [`Option[GetQuestionConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример использования getQuestionConfigs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfigs(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let configs = response.get()
  echo "Received question configs for tenant my-tenant-123"
  echo configs
else:
  echo "No question configs returned"
[inline-code-end]

---