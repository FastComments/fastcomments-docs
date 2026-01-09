---
This endpoint allows you to fetch user badges based on various criteria.

Пример запроса:

[inline-code-attrs-start title = 'Пример GET-запроса'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Вы можете добавить различные параметры запроса для фильтрации результатов:

- `userId` - Получить значки для конкретного пользователя
- `badgeId` - Получить экземпляры конкретного значка
- `type` - Фильтровать по типу значка (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. See UserBadge structure for full list)
- `displayedOnComments` - Фильтровать по тому, отображается ли значок в комментариях (true/false)
- `limit` - Максимальное количество возвращаемых значков (по умолчанию 30, макс 200)
- `skip` - Количество пропускаемых значков (для пагинации)

Пример ответа:

[inline-code-attrs-start title = 'Ответ'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadges": [
    {
      "id": "badge123",
      "userId": "user456",
      "badgeId": "badgeDef789",
      "fromTenantId": "tenant001",
      "createdAt": 1650532511000,
      "receivedAt": 1650532511000,
      "type": 14,
      "name": "Special Contributor",
      "description": "Awarded to special contributors to our community",
      "displayLabel": "Special",
      "backgroundColor": "#4a5568",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 1
    },
    {
      "id": "badge124",
      "userId": "user456",
      "badgeId": "badgeDef790",
      "fromTenantId": "tenant001",
      "createdAt": 1650532598000,
      "receivedAt": 1650532598000,
      "type": 0,
      "threshold": 100,
      "name": "Centurion",
      "description": "Made 100 comments",
      "displayLabel": "100",
      "backgroundColor": "#2b6cb0",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 2
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

[inline-code-attrs-start title = 'Ошибка: недопустимый limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]
---