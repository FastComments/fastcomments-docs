## Параметры

| Name | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Нет |  |
| limit | int | Нет |  |
| tags | seq[string] | Нет |  |
| sso | string | Нет |  |
| isCrawler | bool | Нет |  |
| includeUserInfo | bool | Нет |  |

## Ответ

Возвращает: [`Option[GetFeedPostsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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