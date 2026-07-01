## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | No |  |
| options | DeleteFeedPostPublicOptions | No |  |

## Respons

Returnerer: [`Option[DeleteFeedPostPublicResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public_response.nim)

## Eksempel

[inline-code-attrs-start title = 'deleteFeedPostPublic Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/article-42",
  options = DeleteFeedPostPublicOptions()
)

if response.isSome:
  let deleteResp = response.get()
[inline-code-end]