## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## 示例

[inline-code-attrs-start title = 'blockFromCommentPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockFromCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = "sso-token-7a9b3c"
)
if response.isSome:
  let blockResult = response.get()
  discard blockResult
[inline-code-end]

---