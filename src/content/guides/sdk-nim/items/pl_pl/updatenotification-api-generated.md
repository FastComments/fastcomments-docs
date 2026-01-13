## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Nie |  |
| updateNotificationBody | UpdateNotificationBody | Nie |  |
| userId | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład updateNotification'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateNotification(tenantId = "my-tenant-123",
  id = "notif-456",
  updateNotificationBody = UpdateNotificationBody(),
  userId = "user-789")
if response.isSome:
  let updated = response.get()
  echo "Updated notification id: ", $updated
[inline-code-end]