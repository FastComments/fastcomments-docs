## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | No |  |
| title | string = "" | No |  |

## Одговор

Враћа: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

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
  # користи `page` по потреби
[inline-code-end]