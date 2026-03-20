Този крайна точка ви позволява да извличате потребителски значки въз основа на различни критерии.

Example Request:

[inline-code-attrs-start title = 'Списък на потребителските значки - пример (GET)'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

You can add various query parameters to filter the results:

- `userId` - Вземете значки за конкретен потребител
- `badgeId` - Вземете екземпляри на конкретна значка
- `type` - Филтрирайте по тип на значка (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. See UserBadge structure for full list)
- `displayedOnComments` - Филтрирайте дали значката се показва върху коментарите (true/false)
- `limit` - Максимален брой значки за връщане (default 30, max 200)
- `skip` - Брой значки за пропускане (for pagination)

Example Response:

[inline-code-attrs-start title = 'Отговор'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Possible Error Responses:

[inline-code-attrs-start title = 'Грешка: Липсва Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Грешка: Невалиден Limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]