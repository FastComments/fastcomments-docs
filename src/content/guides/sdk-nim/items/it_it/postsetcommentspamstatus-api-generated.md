## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| commentId | string | Sì |  |
| spam | bool | No |  |
| permNotSpam | bool | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di postSetCommentSpamStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentSpamStatus(
  commentId = "cmt-20250619-842",
  spam = false,
  permNotSpam = false,
  sso = ""
)
if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---