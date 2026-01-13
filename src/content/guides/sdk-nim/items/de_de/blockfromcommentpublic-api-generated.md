## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## Beispiel

[inline-code-attrs-start title = 'blockFromCommentPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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