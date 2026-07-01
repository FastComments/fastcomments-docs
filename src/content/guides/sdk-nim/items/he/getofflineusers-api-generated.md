---
המתגירים הקודמים בדף שאינם מחוברים כעת. ממוין לפי displayName.  
השתמש בזה לאחר שניצלו /users/online כדי להציג חלק "Members".  
דף אינדקס עם cursor על commenterName: השרת מתנהל על החלק החלקי {tenantId, urlId, commenterName}  
מאינדקס אחרי שם באמצעות $gt, ללא עלות $skip.

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## תגובה

מחזיר: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## דוגמא

[inline-code-attrs-start title = 'דוגמת getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]

---