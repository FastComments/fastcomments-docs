## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nej |  |
| afterCreatedAt | int64 | Nej |  |
| unreadOnly | bool | Nej |  |
| dmOnly | bool | Nej |  |
| noDm | bool | Nej |  |
| sso | string | Nej |  |

## Respons

Returnerer: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Eksempel

[inline-code-attrs-start title = 'resetUserNotifications Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  afterId = "",
  afterCreatedAt = 0'i64,
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  sso = ""
)
if response.isSome:
  let resetResp = response.get()
  echo "ResetUserNotificationsResponse received"
else:
  echo "No ResetUserNotificationsResponse"
[inline-code-end]

---