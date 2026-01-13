## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| updateNotificationBody | UpdateNotificationBody | Nee |  |
| userId | string | Nee |  |

## Respons

Retourneert: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'updateNotification Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateNotification(tenantId = "my-tenant-123",
  id = "notif-456",
  updateNotificationBody = UpdateNotificationBody(),
  userId = "user-789")
if response.isSome:
  let updated = response.get()
  echo "Updated notification id: ", $updated
[inline-code-end]

---