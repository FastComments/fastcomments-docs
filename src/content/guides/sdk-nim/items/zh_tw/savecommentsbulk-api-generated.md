## 參數

| 名稱 | 型別 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createCommentParams | seq[CreateCommentParams] | 否 |  |
| isLive | bool | 否 |  |
| doSpamCheck | bool | 否 |  |
| sendEmails | bool | 否 |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | 否 |  |
| id | string | 否 |  |
| fromName | string | 否 |  |

## 回應

回傳：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'saveCommentsBulk 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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