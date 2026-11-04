Ten endpoint pozwala na utworzenie nowego przypisania odznaki użytkownika.

Przykładowe żądanie:

[inline-code-attrs-start title = 'Przykład żądania POST'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "userId": "user456",
  "badgeId": "badgeDef789",
  "displayedOnComments": true
}'
[inline-code-end]

Treść żądania musi zawierać następujące parametry:

- `userId` (wymagany) - ID użytkownika, któremu ma zostać przypisana odznaka
- `badgeId` (wymagany) - ID odznaki do przypisania
- `displayedOnComments` (opcjonalny) - Czy odznaka powinna być wyświetlana przy komentarzach użytkownika (domyślnie true)

Ważne uwagi:
1. Odznaka musi istnieć i być włączona w katalogu odznak Twojej tenanta
2. Możesz przypisywać odznaki tylko użytkownikom, którzy należą do Twojego tenanta lub komentowali na Twojej stronie

Przykładowa odpowiedź:

[inline-code-attrs-start title = 'Odpowiedź'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Możliwe odpowiedzi z błędami:

[inline-code-attrs-start title = 'Błąd: Brak ID tenanta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Błąd: Brak ID użytkownika'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "User ID (body param: userId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Błąd: Brak ID odznaki'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-badge-id",
  "reason": "Badge ID (body param: badgeId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Błąd: Nie znaleziono odznaki'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "badge-not-found",
  "reason": "The badge badgeDef789 was not found or is not enabled."
}
[inline-code-end]

[inline-code-attrs-start title = 'Błąd: Nieautoryzowany użytkownik'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "unauthorized-user",
  "reason": "You can only add badges to users who belong to your tenant or have commented on your site."
}
[inline-code-end]