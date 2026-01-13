Овај endpoint вам омогућава да дохватите корисничке значке на основу различитих критеријума.

Пример захтева:

[inline-code-attrs-start title = 'Пример GET захтева'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Можете додати различите параметре упита да бисте филтрирали резултате:

- `userId` - Добија значке за одређеног корисника
- `badgeId` - Добија инстанце одређене значке
- `type` - Филтрира по типу значке (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, итд. Погледајте структуру UserBadge за пуну листу)
- `displayedOnComments` - Филтрира по томе да ли се значка приказује у коментарима (true/false)
- `limit` - Максималан број значки за враћање (подразумевано 30, максимум 200)
- `skip` - Број значки које треба прескочити (за пагинацију)

Пример одговора:

[inline-code-attrs-start title = 'Одговор'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Могући одговори о грешкама:

[inline-code-attrs-start title = 'Грешка: Недостаје Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Грешка: Неважећи лимит'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]