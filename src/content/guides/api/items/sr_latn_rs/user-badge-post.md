---
Ovaj endpoint vam omogućava da kreirate novu dodelu korisničkog bedža.

Primer zahteva:

[inline-code-attrs-start title = 'Primer POST zahteva'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "userId": "user456",
  "badgeId": "badgeDef789",
  "displayedOnComments": true
}'
[inline-code-end]

Telo zahteva mora da sadrži sledeće parametre:

- `userId` (required) - ID korisnika kome se dodeljuje bedž
- `badgeId` (required) - ID bedža koji se dodeljuje
- `displayedOnComments` (optional) - Da li bedž treba da se prikazuje na korisnikovim komentarima (podrazumevano je true)

Važne napomene:
1. Bedž mora postojati i biti omogućen u katalogu bedževa vašeg tenant-a
2. Bedževe možete dodeljivati samo korisnicima koji pripadaju vašem tenant-u ili su komentarisali na vašem sajtu

Primer odgovora:

[inline-code-attrs-start title = 'Odgovor'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadge": {
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
  }
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

[inline-code-attrs-start title = 'Greška: Nedostaje ID korisnika'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "User ID (body param: userId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Greška: Nedostaje ID bedža'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-badge-id",
  "reason": "Badge ID (body param: badgeId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Greška: Bedž nije pronađen'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "badge-not-found",
  "reason": "The badge badgeDef789 was not found or is not enabled."
}
[inline-code-end]

[inline-code-attrs-start title = 'Greška: Neautorizovan korisnik'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "unauthorized-user",
  "reason": "You can only add badges to users who belong to your tenant or have commented on your site."
}
[inline-code-end]
---