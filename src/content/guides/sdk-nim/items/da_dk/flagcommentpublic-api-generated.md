---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| isFlagged | bool | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Eksempel

[inline-code-attrs-start title = 'flagCommentPublic Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-98765",
  isFlagged = false,
  sso = ""
)
if response.isSome:
  let flagResult = response.get()
  discard flagResult
[inline-code-end]

---