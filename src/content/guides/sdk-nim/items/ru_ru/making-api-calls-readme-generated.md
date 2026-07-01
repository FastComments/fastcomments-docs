Все методы API в этом SDK возвращают кортежи `(Option[ResponseType], Response)`. Первый элемент содержит разобранный ответ при успехе, а второй элемент – необработанный HTTP‑ответ.

Обязательные параметры и тело запроса передаются позиционно. Оставшиеся необязательные параметры собираются в единый объект `Api<Operation>Options`, который является последним аргументом. Операции без необязательных параметров не используют объект опций.

### Пример: Получение комментариев

```nim
import httpclient
import options
import fastcomments
import fastcomments/apis/api_default

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if httpResponse.code == Http200:
  if response.isSome:
    let resp = response.get()
    if resp.comments.isSome:
      echo "Found ", resp.comments.get().len, " comments"
```