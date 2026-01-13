Deze endpoint stelt je in staat gebruikersbadges op te halen op basis van verschillende criteria.

Voorbeeldverzoek:

[inline-code-attrs-start title = 'Voorbeeld GET-verzoek'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Je kunt verschillende queryparameters toevoegen om de resultaten te filteren:

- `userId` - Haal badges op voor een specifieke gebruiker
- `badgeId` - Haal instanties van een specifieke badge op
- `type` - Filter op badgetype (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. Zie de UserBadge-structuur voor de volledige lijst)
- `displayedOnComments` - Filter op of de badge wordt getoond bij reacties (true/false)
- `limit` - Maximaal aantal badges om terug te geven (standaard 30, max 200)
- `skip` - Aantal badges om over te slaan (voor paginering)

Voorbeeldantwoord:

[inline-code-attrs-start title = 'Respons'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Mogelijke foutreacties:

[inline-code-attrs-start title = 'Fout: Ontbrekend Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Fout: Ongeldige limiet'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]