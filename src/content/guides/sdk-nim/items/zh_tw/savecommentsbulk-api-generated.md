## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createCommentParams | seq[CreateCommentParams] | 否 |  |
| isLive | bool | 否 |  |
| doSpamCheck | bool | 否 |  |
| sendEmails | bool | 否 |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | 否 |  |
| id | string | 否 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳：[`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## 範例

[inline-code-attrs-start title = 'saveCommentsBulk 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.saveCommentsBulk(
  tenantId = "my-tenant-123",
  createCommentParams = @[],
  isLive = true,
  doSpamCheck = true,
  sendEmails = false,
  populateNotifications = true,
  id = "batch-20251122",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-456",
  anonUserId = "anon-789"
)
if response.isSome:
  let unblocked = response.get()
  echo "Unblocked response received: ", unblocked
else:
  echo "No unblocked response, httpResponse: ", $httpResponse
[inline-code-end]

---