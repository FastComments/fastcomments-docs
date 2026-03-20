Ten endpoint pozwala pobrać odznaki użytkowników na podstawie różnych kryteriów.

Przykładowe żądanie:

[inline-code-attrs-start title = 'Lista odznak użytkownika - przykład GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Możesz dodać różne parametry zapytania, aby filtrować wyniki:

- `userId` - Pobierz odznaki dla konkretnego użytkownika
- `badgeId` - Pobierz instancje określonej odznaki
- `type` - Filtruj według typu odznaki (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. See UserBadge structure for full list)
- `displayedOnComments` - Filtruj według tego, czy odznaka jest wyświetlana przy komentarzach (true/false)
- `limit` - Maksymalna liczba zwracanych odznak (domyślnie 30, maksymalnie 200)
- `skip` - Liczba odznak do pominięcia (do paginacji)

Przykładowa odpowiedź:

[inline-code-attrs-start title = 'Odpowiedź'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Możliwe odpowiedzi błędów:

[inline-code-attrs-start title = 'Błąd: Brak identyfikatora Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Błąd: Nieprawidłowy limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]