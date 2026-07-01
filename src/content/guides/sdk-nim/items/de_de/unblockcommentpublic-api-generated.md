## Parameter

| Name | Type | Required | Beschreibung |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Nein |  |
| sso | string = "" | Nein |  |

## Antwort

Rückgabe: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Beispiel

[inline-code-attrs-start title = 'unBlockCommentPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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