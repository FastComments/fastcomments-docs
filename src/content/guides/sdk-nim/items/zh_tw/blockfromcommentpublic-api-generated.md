## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## 範例

[inline-code-attrs-start title = 'blockFromCommentPublic 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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