## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostBanUserFromCommentOptions | No |  |

## Yanıt

Döndürür: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Örnek

[inline-code-attrs-start title = 'postBanUserFromComment Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (banResult, httpResp) = client.postBanUserFromComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  options = PostBanUserFromCommentOptions()
)

if banResult.isSome:
  let result = banResult.get()
  echo result
[inline-code-end]