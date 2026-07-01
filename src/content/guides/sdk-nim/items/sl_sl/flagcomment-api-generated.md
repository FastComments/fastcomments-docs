## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| options | FlagCommentOptions | Ne |  |

## Odgovor

Vrne: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer flagComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let options = FlagCommentOptions(reason: "spam content", note: "Automated posting detected", isSpam: true, categories: @["spam"])
let (flagRes, httpRes) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-789", options = options)
if flagRes.isSome:
  let res = flagRes.get()
  discard res
[inline-code-end]

---