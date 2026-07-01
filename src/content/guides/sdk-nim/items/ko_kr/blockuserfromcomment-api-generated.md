## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| blockFromCommentParams | BlockFromCommentParams | 아니오 |  |
| options | BlockUserFromCommentOptions | 아니오 |  |

## Response

반환: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Example

[inline-code-attrs-start title = 'blockUserFromComment 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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