## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| pageSize | int | Nie |  |
| afterId | string | Nie |  |
| includeContext | bool | Nie |  |
| afterCreatedAt | int64 | Nie |  |
| unreadOnly | bool | Nie |  |
| dmOnly | bool | Nie |  |
| noDm | bool | Nie |  |
| includeTranslations | bool | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  pageSize = 50,
  afterId = "notif_9a1b2c3d",
  includeContext = true,
  afterCreatedAt = int64(1699999999000),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  sso = ""
)
if response.isSome:
  let notifications = response.get()
  discard notifications
else:
  discard httpResponse
[inline-code-end]

---