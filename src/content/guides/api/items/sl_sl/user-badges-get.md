Ta končna točka omogoča pridobivanje uporabniških značk glede na različna merila.

Primer zahteve:

[inline-code-attrs-start title = 'Seznam uporabniških značk - primer GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Lahko dodate različne query parametre za filtriranje rezultatov:

- `userId` - Pridobi značke za določenega uporabnika
- `badgeId` - Pridobi primere določene značke
- `type` - Filtriraj po tipu značke (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. Oglejte si strukturo UserBadge za celoten seznam)
- `displayedOnComments` - Filtriraj glede na to, ali je značka prikazana pri komentarjih (true/false)
- `limit` - Maksimalno število značk, ki se vrnejo (privzeto 30, max 200)
- `skip` - Število značk za preskočiti (za paginacijo)

Primer odgovora:

[inline-code-attrs-start title = 'Odgovor'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Možni odgovori z napakami:

[inline-code-attrs-start title = 'Napaka: manjkajoči ID najemnika'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Napaka: neveljaven limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]