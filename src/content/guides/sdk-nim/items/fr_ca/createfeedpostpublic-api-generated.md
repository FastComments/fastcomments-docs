## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| createFeedPostParams | CreateFeedPostParams | Non |  |
| options | CreateFeedPostPublicOptions | Non |  |

## Réponse

Renvoie : [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple createFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = CreateFeedPostParams(
  urlId: "news/big-event",
  title: "Big Event Happened",
  content: "Full article content goes here.",
  tags: @["news", "event"]
)

let opts = CreateFeedPostPublicOptions(
  sendNotifications: false,
  allowComments: true
)

let (response, httpResponse) = client.createFeedPostPublic(
  tenantId = "my-tenant-123",
  createFeedPostParams = params,
  options = opts
)

if response.isSome:
  let post = response.get()
  echo post
[inline-code-end]