## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postId | string | Ne |  |
| reactBodyParams | ReactBodyParams | Ne |  |
| isUndo | bool | Ne |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[ReactFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_public200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer reactFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/article-title",
  reactBodyParams = ReactBodyParams(),
  isUndo = false,
  broadcastId = "broadcast-456",
  sso = ""
)
if response.isSome:
  let result = response.get()
  echo "Reaction result: ", result
else:
  echo "Reaction failed, HTTP response: ", httpResponse
[inline-code-end]

---