## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| commentId | string | Ναι |  |
| reviewed | bool | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'postSetCommentReviewStatus Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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