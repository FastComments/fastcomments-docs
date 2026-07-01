## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createAPIPageData | CreateAPIPageData | Нет |  |

## Ответ

Возвращает: [`Option[AddPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_page_api_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример addPage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let pageData = CreateAPIPageData(
  urlId = "news/article-2024",
  title = "Breaking News: Nim Takes Over",
  description = "An in-depth article about Nim's rise.",
  tags = @["nim", "programming", "news"]
)

let (addPageResp, httpResp) = client.addPage(
  tenantId = "my-tenant-123",
  createAPIPageData = pageData
)

if addPageResp.isSome:
  let resp = addPageResp.get()
  echo resp
[inline-code-end]