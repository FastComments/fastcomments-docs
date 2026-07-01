## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|------|------|
| tenantId | string | Yes |  |
| id | string | No |  |
| blockFromCommentParams | BlockFromCommentParams | No |  |
| options | BlockUserFromCommentOptions | No |  |

## レスポンス

戻り値: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## 例

[inline-code-attrs-start title = 'blockUserFromComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = BlockFromCommentParams()
let opts = BlockUserFromCommentOptions()
let (blockResult, httpResp) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  blockFromCommentParams = params,
  options = opts
)
if blockResult.isSome:
  let success = blockResult.get()
  discard success
[inline-code-end]