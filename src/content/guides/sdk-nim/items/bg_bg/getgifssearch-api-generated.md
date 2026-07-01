## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| search | string | Не |  |
| options | GetGifsSearchOptions | Не |  |

## Отговор

Връща: [`Option[GetGifsSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_search_response.nim)

## Пример

[inline-code-attrs-start title = 'getGifsSearch Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetGifsSearchOptions(limit = 10, rating = "g")
let (responseOpt, httpResponse) = client.getGifsSearch(tenantId = "my-tenant-123", search = "funny cats", options = opts)
if responseOpt.isSome:
  let resp = responseOpt.get()
  # използвай resp, както е необходимо
[inline-code-end]

---