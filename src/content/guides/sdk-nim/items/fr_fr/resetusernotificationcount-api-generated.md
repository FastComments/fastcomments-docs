## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| sso | string = "" | Non |  |

## Réponse

Retourne : [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de resetUserNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (resetRespOpt, httpResp) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "user-456")
if resetRespOpt.isSome:
  let resetResp = resetRespOpt.get()
  echo resetResp
else:
  echo "Reset notification count response not available"
[inline-code-end]