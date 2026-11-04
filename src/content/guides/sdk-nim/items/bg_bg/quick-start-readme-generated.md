### Използване на автентикирани API (DefaultAPI)

**Важно:** Автентикираните крайни точки изискват вашият API ключ да бъде зададен като заглавка `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Извършване на автентикирани API повиквания.
# Задължителните параметри (и тялото на заявката) са позиционни; незадължителните
# параметри се подават чрез обекта с опции на операцията.
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

### Използване на публични API (PublicAPI)

Публичните крайни точки не изискват автентикация:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Извършване на публични API повиквания.
# tenantId и urlId са задължителни (позиционни); всичко останало е незадължително.
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

### Използване на API за модериране (ModerationAPI)

Крайните точки за модериране захранват таблото на модератора и се автентикират със SSO токен за действащия модератор:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Изброяване на коментари в таблото за модериране.
# Тази операция няма задължителни параметри, така че всичко е незадължително.
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

### Чести проблеми

1. **Грешка при автентикация 401**: Уверете се, че сте задали заглавката `x-api-key` на вашия HttpClient, преди да извършвате заявки към DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Грешен API клас**: Използвайте `api_default` за сървърни автентикирани заявки, `api_public` за клиентски/публични заявки и `api_moderation` за заявки към таблото на модератора.