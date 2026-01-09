## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Не |  |
| limit | int | Не |  |
| tags | seq[string] | Не |  |
| sso | string | Не |  |
| isCrawler | bool | Не |  |
| includeUserInfo | bool | Не |  |

## Отговор

Връща: [`Option[GetFeedPostsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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