## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## 範例

[inline-code-attrs-start title = 'unFlagComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unFlagComment(tenantId = "my-tenant-123",
  id = "comment-98765",
  userId = "user-12345",
  anonUserId = "")

if response.isSome:
  let flagResp = response.get()
  echo "Unflagged comment response:", flagResp
else:
  echo "Unflag failed, HTTP status:", httpResponse.status
[inline-code-end]

---