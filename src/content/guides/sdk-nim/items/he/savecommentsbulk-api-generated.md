## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createCommentParams | seq[CreateCommentParams] | לא |  |
| isLive | bool | לא |  |
| doSpamCheck | bool | לא |  |
| sendEmails | bool | לא |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | לא |  |
| id | string | לא |  |
| fromName | string | לא |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.saveCommentsBulk(
  tenantId = "my-tenant-123",
  createCommentParams = @[],
  isLive = false,
  doSpamCheck = false,
  sendEmails = false,
  populateNotifications = false,
  id = "",
  fromName = ""
)

if response.isSome:
  let apiResp = response.get()
  echo "Bulk save succeeded, tenant:", " my-tenant-123"
else:
  echo "Bulk save returned no API response"
[inline-code-end]

---