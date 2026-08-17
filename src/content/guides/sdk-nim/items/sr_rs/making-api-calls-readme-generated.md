Све API методе у овом SDK‑у враћају торке `(Option[ResponseType], Response)`. Први елемент садржи парсиран одговор ако је успео, а други елемент је сирови HTTP одговор.

Обавезни параметри и тело захтева се прослеђују позиционално. Преостали опциона параметри се прикупљају у један објекат `Api<Operation>Options`, који је последњи аргумент. Операције без опциона параметара не узимају објекат опција.

### Пример: Прибављање коментара

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