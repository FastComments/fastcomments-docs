Всички API методи в този SDK връщат кортежи от `(Option[ResponseType], Response)`. Първият елемент съдържа парснатия отговор, ако е успешен, а вторият елемент е суровият HTTP отговор.

Задължителните параметри и тялото на заявката се предават позиционно. Останалите незадължителни параметри се събират в един единствен обект `Api<Operation>Options`, който е последният аргумент. Операциите без незадължителни параметри не използват обект за опции.

### Пример: Извличане на коментари

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