## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createCommentParams | seq[CreateCommentParams] | いいえ |  |
| isLive | bool | いいえ |  |
| doSpamCheck | bool | いいえ |  |
| sendEmails | bool | いいえ |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | いいえ |  |
| id | string | いいえ |  |
| fromName | string | いいえ |  |

## レスポンス

戻り値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'saveCommentsBulk の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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