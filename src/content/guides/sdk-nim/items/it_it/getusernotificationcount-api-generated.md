## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[GetUserNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotificationCount(tenantId = "news-tenant-123", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTI0NyIsImlhdCI6MTYw945600fQ.sflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c")
if response.isSome:
  let countResp = response.get()
  echo "Received user notification count response: ", countResp
else:
  echo "No notification count returned"
[inline-code-end]

---