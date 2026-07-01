## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| postId | string | Ne |  |
| options | DeleteFeedPostPublicOptions | Ne |  |

## Odgovor

Vraća: [`Option[DeleteFeedPostPublicResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public_response.nim)

## Primer

[inline-code-attrs-start title = 'deleteFeedPostPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/article-42",
  options = DeleteFeedPostPublicOptions()
)

if response.isSome:
  let deleteResp = response.get()
[inline-code-end]