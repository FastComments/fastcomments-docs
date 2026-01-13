## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |
| updateNotificationBody | UpdateNotificationBody | Hayır |  |
| userId | string | Hayır |  |

## Yanıt

Döndürür: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Örnek

[inline-code-attrs-start title = 'updateNotification Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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