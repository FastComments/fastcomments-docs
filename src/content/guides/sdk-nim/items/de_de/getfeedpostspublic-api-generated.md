## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nein |  |
| limit | int | Nein |  |
| tags | seq[string] | Nein |  |
| sso | string | Nein |  |
| isCrawler | bool | Nein |  |
| includeUserInfo | bool | Nein |  |

## Antwort

Gibt zurück: [`Option[GetFeedPostsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_public200response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsPublic(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[],
  sso = "",
  isCrawler = false,
  includeUserInfo = false
)

if response.isSome:
  let feed = response.get()
  echo feed
[inline-code-end]

---