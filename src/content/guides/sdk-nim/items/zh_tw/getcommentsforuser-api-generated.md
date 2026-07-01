## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| options | GetCommentsForUserOptions | No |  |

## 回應

回傳: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## 範例

[inline-code-attrs-start title = 'getCommentsForUser 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getCommentsForUser(options = GetCommentsForUserOptions(
  tenantId = "my-tenant-123",
  userId = "user-456",
  page = 0,
  pageSize = 20,
  includeDeleted = false,
  commentIds = @[]
))

if maybeResp.isSome:
  let resp = maybeResp.get()
  discard resp
[inline-code-end]