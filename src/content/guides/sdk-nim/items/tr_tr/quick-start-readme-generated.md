### Kimlik Doğrulamalı API'leri Kullanma (DefaultAPI)

**Önemli:** Kimlik doğrulamalı uç noktalar API anahtarınızın `x-api-key` başlığı olarak ayarlanmasını gerektirir.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Kimlik doğrulamalı API çağrıları yapın.
# Gerekli parametreler (ve istek gövdesi) konumsaldır; isteğe bağlı
# parametreler operasyonun seçenekler nesnesi aracılığıyla iletilir.
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### Genel API'leri Kullanma (PublicAPI)

Genel uç noktalar kimlik doğrulama gerektirmez:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Genel API çağrıları yapın.
# tenantId ve urlId gereklidir (konumsal); diğer tüm parametreler isteğe bağlıdır.
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  options = GetCommentsPublicOptions(
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### Moderasyon API'lerini Kullanma (ModerationAPI)

Moderasyon uç noktaları moderatör panelini çalıştırır ve işlemi yapan moderatör için bir SSO tokenı ile kimlik doğrulaması yapılır:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Moderasyon panelindeki yorumları listele.
# Bu işlem için gerekli parametre yoktur, bu yüzden tümü isteğe bağlıdır.
let (response, httpResponse) = getApiComments(
  httpClient = client,
  options = GetApiCommentsOptions(
    count: 30,
    tenantId: "your-tenant-id",
    sso: "your-sso-token"
  )
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### Yaygın Sorunlar

1. **401 kimlik doğrulama hatası**: DefaultAPI istekleri yapmadan önce HttpClient'ınızda `x-api-key` başlığını ayarladığınızdan emin olun: `client.headers["x-api-key"] = "your-api-key"`
2. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `api_default`, istemci/halka açık istekler için `api_public` ve moderatör paneli istekleri için `api_moderation` kullanın.