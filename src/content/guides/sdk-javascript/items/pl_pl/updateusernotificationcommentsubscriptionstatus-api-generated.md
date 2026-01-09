---
Włącz lub wyłącz powiadomienia dla konkretnego komentarza.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| notificationId | string | Tak |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Tak |  |
| commentId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

---