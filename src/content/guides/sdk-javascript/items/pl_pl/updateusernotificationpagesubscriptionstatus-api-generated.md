Włącz lub wyłącz powiadomienia dla strony. Gdy użytkownicy subskrybują stronę, tworzone są powiadomienia dla nowych komentarzy głównych, a także

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| url | string | Tak |  |
| pageTitle | string | Tak |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)