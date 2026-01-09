## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di resetUserNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "sso-jwt-9a8b7c6d")
if response.isSome:
  let resetResult = response.get()
  echo resetResult
else:
  echo "Reset failed, status: ", httpResponse.status
[inline-code-end]

---