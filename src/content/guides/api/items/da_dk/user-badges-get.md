Dette endepunkt giver dig mulighed for at hente bruger-badges baseret på forskellige kriterier.

Example Request:

[inline-code-attrs-start title = 'Liste over bruger-badges - GET-eksempel'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Du kan tilføje forskellige forespørgselsparametre for at filtrere resultaterne:

- `userId` - Få badges for en specifik bruger
- `badgeId` - Få forekomster af et specifikt badge
- `type` - Filtrer efter badge-type (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, osv. Se UserBadge-strukturen for den fulde liste)
- `displayedOnComments` - Filtrer efter om badge vises på kommentarer (true/false)
- `limit` - Maksimalt antal badges der returneres (standard 30, maks 200)
- `skip` - Antal badges der springes over (til paginering)

Example Response:

[inline-code-attrs-start title = 'Svar'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Mulige fejlresponser:

[inline-code-attrs-start title = 'Fejl: Manglende lejer-id'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Fejl: Ugyldig grænse'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]