## Parameters

| Name | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'blockFromCommentPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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