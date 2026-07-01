## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createFeedPostParams | CreateFeedPostParams | Nej |  |
| options | CreateFeedPostOptions | Nej |  |

## Respons

Returnerer: [`Option[CreateFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_posts_response.nim)

## Eksempel

[inline-code-attrs-start title = 'createFeedPost Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(),
  options = CreateFeedPostOptions()
)

if respOpt.isSome:
  let feedPost = respOpt.get()
[inline-code-end]