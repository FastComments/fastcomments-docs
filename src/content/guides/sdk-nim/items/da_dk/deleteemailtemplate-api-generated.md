## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nej |  |

## Svar

Returnerer: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på deleteEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplate(tenantId = "my-tenant-123", id = "tmpl-456")
if response.isSome:
  let deleted = response.get()
  echo deleted
[inline-code-end]