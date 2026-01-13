---
Meldingen voor een specifieke opmerking in- of uitschakelen.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| notificationId | string | Ja |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Ja |  |
| commentId | string | Ja |  |
| sso | string | Nee |  |

## Antwoord

Retourneert: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

---