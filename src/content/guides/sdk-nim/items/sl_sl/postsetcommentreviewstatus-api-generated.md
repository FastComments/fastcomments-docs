## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| options | PostSetCommentReviewStatusOptions | Ne |  |

## Odgovor

Vrne: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'postSetCommentReviewStatus Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.postSetCommentReviewStatus(
  tenantId = "my-tenant-123",
  commentId = "cmt-7890",
  options = PostSetCommentReviewStatusOptions()
)

if apiResp.isSome:
  let _ = apiResp.get()
  discard
else:
  discard
[inline-code-end]