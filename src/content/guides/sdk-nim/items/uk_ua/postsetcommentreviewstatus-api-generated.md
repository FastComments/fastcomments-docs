## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| options | PostSetCommentReviewStatusOptions | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'postSetCommentReviewStatus Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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