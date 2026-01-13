## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| yearNumber | float64 | Нет |  |
| monthNumber | float64 | Нет |  |
| dayNumber | float64 | Нет |  |
| skip | float64 | Нет |  |

## Ответ

Возвращает: [`Option[GetTenantDailyUsages_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getTenantDailyUsages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantDailyUsages(
  tenantId = "my-tenant-123",
  yearNumber = 2025.0,
  monthNumber = 6.0,
  dayNumber = 15.0,
  skip = 0.0
)

if response.isSome:
  let usages = response.get()
  echo usages
else:
  echo "No daily usages returned"
[inline-code-end]