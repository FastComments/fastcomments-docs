## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nee |  |
| afterCreatedAt | int64 | Nee |  |
| unreadOnly | bool | Nee |  |
| dmOnly | bool | Nee |  |
| noDm | bool | Nee |  |
| sso | string | Nee |  |

## Respons

Geeft terug: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'resetUserNotifications Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  afterId = "",
  afterCreatedAt = int64(0),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  sso = ""
)

if response.isSome:
  let result = response.get()
[inline-code-end]

---