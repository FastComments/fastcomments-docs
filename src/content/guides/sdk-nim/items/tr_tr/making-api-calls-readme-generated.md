Bu SDK'daki tüm API metotları `(Option[ResponseType], Response)` şeklinde demetler (tuple) döndürür. İlk öğe başarılı olması durumunda ayrıştırılmış yanıtı, ikinci öğe ise ham HTTP yanıtını içerir.

Zorunlu parametreler ve istek gövdesi konumsal olarak iletilir. Kalan isteğe bağlı parametreler, son argüman olan tek bir `Api<Operation>Options` nesnesinde toplanır. İsteğe bağlı parametresi olmayan işlemler options nesnesi almaz.

### Örnek: Yorumları Getirme

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