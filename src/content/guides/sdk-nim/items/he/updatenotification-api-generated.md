## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| updateNotificationBody | UpdateNotificationBody | לא |  |
| userId | string | לא |  |

## תגובה

מחזיר: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateNotification'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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