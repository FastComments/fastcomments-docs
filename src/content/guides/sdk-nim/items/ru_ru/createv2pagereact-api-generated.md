## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|-------------|----------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| id | string | Нет |  |
| title | string = "" | Нет |  |

## Ответ

Возвращает: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Пример

[inline-code-attrs-start title = 'createV2PageReact Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pageResult, httpResponse) = client.createV2PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  id = "page-456",
  title = "Breaking News",
)

if pageResult.isSome:
  let page = pageResult.get()
  # использовать `page` по необходимости
[inline-code-end]