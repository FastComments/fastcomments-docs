## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| postId | string | Ne |  |
| updateFeedPostParams | UpdateFeedPostParams | Ne |  |
| options | UpdateFeedPostPublicOptions | Ne |  |

## Odgovor

Vraća: [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Primer

[inline-code-attrs-start title = 'updateFeedPostPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-789",
  updateFeedPostParams = UpdateFeedPostParams(),
  options = UpdateFeedPostPublicOptions()
)

if response.isSome:
  let post = response.get()
[inline-code-end]