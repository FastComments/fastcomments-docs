## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| commentId | string | Ja |  |
| spam | bool | Nee |  |
| permNotSpam | bool | Nee |  |
| sso | string | Nee |  |

## Response

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postSetCommentSpamStatus Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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