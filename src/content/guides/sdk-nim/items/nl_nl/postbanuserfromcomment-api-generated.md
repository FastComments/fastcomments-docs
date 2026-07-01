## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| options | PostBanUserFromCommentOptions | Nee |  |

## Reactie

Retourneert: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postBanUserFromComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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