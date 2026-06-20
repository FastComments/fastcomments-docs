## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de resetUserNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "user-sso-token-456")
if response.isSome:
  let result = response.get()
  echo "ResetUserNotificationsResponse:", result
else:
  echo "Reset failed, HTTP response:", httpResponse
[inline-code-end]

---