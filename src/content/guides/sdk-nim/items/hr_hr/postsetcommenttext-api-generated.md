## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| setCommentTextParams | SetCommentTextParams | Ne |  |
| options | PostSetCommentTextOptions | Ne |  |

## Odgovor

Vraća: [`Option[SetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text_response.nim)

## Primjer

[inline-code-attrs-start title = 'postSetCommentText Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.postSetCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  setCommentTextParams = SetCommentTextParams(),
  options = PostSetCommentTextOptions()
)

if responseOpt.isSome:
  let updatedComment = responseOpt.get()
[inline-code-end]

---