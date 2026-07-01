## Parameters

| Naam | Type | Vereist | Omschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postId | string | Nee |  |
| updateFeedPostParams | UpdateFeedPostParams | Nee |  |
| options | UpdateFeedPostPublicOptions | Nee |  |

## Respons

Retourneert: [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'updateFeedPostPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---