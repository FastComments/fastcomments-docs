Этот endpoint позволяет получить записи прогресса бейджей пользователей на основе различных критериев.

Пример запроса:

[inline-code-attrs-start title = 'Список прогресса бейджей — пример GET-запроса'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Вы можете добавить различные query-параметры для фильтрации результатов:

- `userId` - Получить прогресс для конкретного пользователя
- `limit` - Максимальное количество записей для возвращения (по умолчанию 30, максимум 200)
- `skip` - Количество записей для пропуска (для постраничной навигации)

Пример ответа:

[inline-code-attrs-start title = 'Ответ'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgresses": [
    {
      "id": "progress123",
      "tenantId": "tenant001",
      "userId": "user456",
      "firstCommentId": "comment789",
      "firstCommentDate": 1650532511000,
      "autoTrustFactor": 0.75,
      "progress": {
        "0": 42,
        "1": 120,
        "2": 15,
        "3": 3,
        "5": 5,
        "6": 1800000,
        "8": 0,
        "7": 0
      }
    },
    {
      "id": "progress124",
      "tenantId": "tenant001",
      "userId": "user789",
      "firstCommentId": "comment790",
      "firstCommentDate": 1650532598000,
      "autoTrustFactor": 0.5,
      "progress": {
        "0": 12,
        "1": 15,
        "2": 4
      }
    }
  ]
}
[inline-code-end]

Возможные ответы об ошибке:

[inline-code-attrs-start title = 'Ошибка: отсутствует Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Ошибка: недопустимый параметр limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]