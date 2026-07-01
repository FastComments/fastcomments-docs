## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string = "" | No |  |

## 回應

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'lockComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (lockResult, httpRes) = client.lockComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  broadcastId = "",
  sso = "")

if lockResult.isSome:
  let resp = lockResult.get()
  discard resp
[inline-code-end]