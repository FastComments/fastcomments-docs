## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| reviewed | bool | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Örnek

[inline-code-attrs-start title = 'postSetCommentReviewStatus Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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