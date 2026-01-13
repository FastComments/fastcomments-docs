## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tag | string | Nej |  |
| tenantId | string | Ja |  |
| deleteHashTagRequest | DeleteHashTagRequest | Nej |  |

## Svar

Returnerer: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Eksempel

[inline-code-attrs-start title = 'deleteHashTag Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(tag = "breaking-news", tenantId = "my-tenant-123", deleteHashTagRequest = DeleteHashTagRequest())
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---