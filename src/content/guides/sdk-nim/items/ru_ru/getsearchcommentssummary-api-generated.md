## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| value | string | Нет |  |
| filters | string | Нет |  |
| searchFilters | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[ModerationCommentSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_comment_search_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример использования getSearchCommentsSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchCommentsSummary(
  value = "news/climate-change-2026",
  filters = "{\"tenantId\":\"my-tenant-123\",\"siteId\":\"main-site\",\"status\":\"approved\"}",
  searchFilters = "author:journalist@news.com OR content:climate",
  sso = "sso-xyz-7890"
)
if response.isSome:
  let summary = response.get()
  echo "Received summary: ", $summary
else:
  echo "No summary returned, HTTP response: ", $httpResponse
[inline-code-end]

---