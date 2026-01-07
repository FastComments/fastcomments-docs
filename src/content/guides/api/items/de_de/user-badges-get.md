Dieser Endpunkt ermöglicht das Abrufen von Benutzer-Badges basierend auf verschiedenen Kriterien.

Beispielanfrage:

[inline-code-attrs-start title = 'GET Anfrage Beispiel'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Sie können verschiedene Abfrageparameter hinzufügen, um die Ergebnisse zu filtern:

- `userId` - Badges für einen bestimmten Benutzer abrufen
- `badgeId` - Instanzen eines bestimmten Badges abrufen
- `type` - Nach Badge-Typ filtern (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, usw. Siehe UserBadge-Struktur für vollständige Liste)
- `displayedOnComments` - Filtern, ob das Badge bei Kommentaren angezeigt wird (true/false)
- `limit` - Maximale Anzahl der zurückzugebenden Badges (Standard 30, max 200)
- `skip` - Anzahl der zu überspringenden Badges (für Paginierung)

Beispielantwort:

[inline-code-attrs-start title = 'Antwort'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Mögliche Fehlerantworten:

[inline-code-attrs-start title = 'Fehler: Fehlende Tenant-ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Fehler: Ungültiges Limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]
