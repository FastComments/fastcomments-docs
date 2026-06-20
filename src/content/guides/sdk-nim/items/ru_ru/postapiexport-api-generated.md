## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| textSearch | string | Нет |  |
| byIPFromComment | string | Нет |  |
| filters | string | Нет |  |
| searchFilters | string | Нет |  |
| sorts | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[ModerationExportResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример postApiExport'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postApiExport(
  textSearch = "offensive language and spam",
  byIPFromComment = "203.0.113.45",
  filters = "{\"status\":\"pending\",\"severity\":\"high\"}",
  searchFilters = "authorEmail:editor@news-site.com",
  sorts = "-createdAt",
  sso = "sso-session-token-9f8b7c"
)
if response.isSome:
  let exportResp = response.get()
  echo "Moderation export received:", exportResp
else:
  echo "No export returned, HTTP status:", httpResponse.status.code
[inline-code-end]

---