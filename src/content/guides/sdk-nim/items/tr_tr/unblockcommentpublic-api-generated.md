## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | No |  |
| sso | string = "" | No |  |

## Yanıt

Döndürür: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Örnek

[inline-code-attrs-start title = 'unBlockCommentPublic Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (unblockResult, httpResp) = client.unBlockCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-7890",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = ""
)

if unblockResult.isSome:
  let result = unblockResult.get()
[inline-code-end]