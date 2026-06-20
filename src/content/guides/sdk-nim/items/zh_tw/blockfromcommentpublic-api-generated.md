## 參數

| Name | Type | 必填 | Description |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## 範例

[inline-code-attrs-start title = 'blockFromCommentPublic 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let publicParams = PublicBlockFromCommentParams(
  reason = "Repeated spam links",
  durationMinutes = 1440,
  blockAll = true,
  notifyUser = false,
  tags = @["spam", "auto-block"]
)

let (response, httpResponse) = client.blockFromCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-98765",
  publicBlockFromCommentParams = publicParams,
  sso = ""
)

if response.isSome:
  let blockResult = response.get()
  echo "Block succeeded: ", $blockResult
else:
  echo "Block failed, HTTP status: ", $httpResponse.status
[inline-code-end]

---