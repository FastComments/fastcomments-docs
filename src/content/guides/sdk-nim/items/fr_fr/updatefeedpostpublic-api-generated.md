## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| postId | string | Non |  |
| updateFeedPostParams | UpdateFeedPostParams | Non |  |
| broadcastId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'updateFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-456",
  updateFeedPostParams = UpdateFeedPostParams(title = "Weekly Product Update", content = "Released bug fixes and performance improvements in v2.1.", tags = @["release", "product"], pinned = false),
  broadcastId = "",
  sso = ""
)
if response.isSome:
  let created = response.get()
  echo "Updated feed post id: ", created.postId
else:
  echo "Update failed with HTTP status: ", httpResponse.status
[inline-code-end]

---