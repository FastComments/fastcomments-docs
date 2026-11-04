Ovaj endpoint vam omogućava da dohvatite korisničke bedževe na osnovu različitih kriterijuma.

Primer zahteva:

[inline-code-attrs-start title = 'Listanje korisničkih bedževa - GET primer'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Možete dodati različite query parametre za filtriranje rezultata:

- `userId` - Dohvatanje bedževa za određenog korisnika
- `badgeId` - Dohvatanje instanci određenog bedža
- `type` - Filtriranje po tipu bedža (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, itd. Pogledajte strukturu UserBadge za kompletnu listu)
- `displayedOnComments` - Filtriranje po tome da li se bedž prikazuje na komentarima (true/false)
- `limit` - Maksimalan broj bedževa za vraćanje (podrazumevano 30, maksimum 200)
- `skip` - Broj bedževa koje treba preskočiti (za paginaciju)

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

Mogući odgovori sa greškom:

[inline-code-attrs-start title = 'Greška: Nedostaje Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Greška: Nevažeći limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]