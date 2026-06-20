## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| blockFromCommentParams | BlockFromCommentParams | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回：[`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## 示例

[inline-code-attrs-start title = 'blockUserFromComment 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "cmt-7890",
  blockFromCommentParams = BlockFromCommentParams(
    reason = "Repeated abusive language",
    durationMinutes = 1440,
    notifyUser = true,
    tags = @["abuse", "automated"]
  ),
  userId = "user-456",
  anonUserId = ""
)

if response.isSome:
  let result = response.get()
  discard result
else:
  discard httpResponse
[inline-code-end]

---