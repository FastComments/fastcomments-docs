## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | FlagCommentOptions | No |  |

## Risposta

Returns: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio flagComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let options = FlagCommentOptions(reason: "spam content", note: "Automated posting detected", isSpam: true, categories: @["spam"])
let (flagRes, httpRes) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-789", options = options)
if flagRes.isSome:
  let res = flagRes.get()
  discard res
[inline-code-end]