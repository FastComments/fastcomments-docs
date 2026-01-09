Ovaj endpoint omogućuje dohvat korisničkih znački na temelju raznih kriterija.

Primjer zahtjeva:

[inline-code-attrs-start title = 'Primjer GET zahtjeva'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Možete dodati različite parametre upita za filtriranje rezultata:

- `userId` - Dohvati značke za određenog korisnika
- `badgeId` - Dohvati instance određene značke
- `type` - Filtriraj po tipu značke (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. See UserBadge structure for full list)
- `displayedOnComments` - Filtriraj prema tome je li značka prikazana u komentarima (true/false)
- `limit` - Maksimalan broj znački za vraćanje (zadano 30, max 200)
- `skip` - Broj znački koje treba preskočiti (za paginaciju)

Primjer odgovora:

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

Mogući odgovori s pogreškom:

[inline-code-attrs-start title = 'Pogreška: Nedostaje Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Pogreška: Neispravan limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]