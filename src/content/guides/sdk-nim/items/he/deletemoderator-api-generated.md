## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| sendEmail | string = "" | No |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'deleteModerator דוגמה'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteModerator(
  tenantId = "my-tenant-123",
  id = "mod-789",
  sendEmail = "admin@mydomain.com",
)

if apiResp.isSome:
  let empty = apiResp.get()
  echo "Moderator removed"
[inline-code-end]