Усі методи API в цьому SDK повертають кортежі `(Option[ResponseType], Response)`. Перший елемент містить розпарсений відповідь, якщо успішно, а другий елемент — це необроблена HTTP‑відповідь.

Обов’язкові параметри та тіло запиту передаються позиційно. Решта необов’язкових параметрів збираються в один об’єкт `Api<Operation>Options`, який є останнім аргументом. Операції без необов’язкових параметрів не беруть об’єкт параметрів.

### Приклад: отримання коментарів

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
---