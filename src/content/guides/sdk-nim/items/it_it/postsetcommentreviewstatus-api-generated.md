## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| commentId | string | Sì |  |
| reviewed | bool | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di postSetCommentReviewStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentReviewStatus(
  commentId = "cmt-98765-news-article",
  reviewed = false,
  sso = ""
)
if response.isSome:
  let apiResp = response.get()
  echo "Review status updated"
else:
  echo "Failed to update review status: " & $httpResponse.status
[inline-code-end]

---