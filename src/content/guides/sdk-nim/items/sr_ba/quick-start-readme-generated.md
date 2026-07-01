### Korištenje autentifikovanih API-ja (DefaultAPI)

**Važno:** Autentifikovani endpointi zahtijevaju da vaš API ključ bude postavljen kao `x-api-key` zaglavlje.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Napravite autentifikovane API pozive.
# Potrebni parametri (i tijelo zahtjeva) su pozicioni; opcionalni
# parametri se prosljeđuju kroz options objekt operacije.
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

### Korištenje javnih API-ja (PublicAPI)

Javni endpointi ne zahtijevaju autentifikaciju:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Napravite javne API pozive.
# tenantId i urlId su potrebni (pozicioni); sve ostalo je opcionalno.
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

### Korištenje moderacijskih API-ja (ModerationAPI)

Moderacijski endpointi pogone moderator dashboard i autentifikovani su SSO tokenom za djelujućeg moderatora:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Prikažite komentare na moderacijskom dashboardu.
# Ova operacija nema potrebnih parametara, pa je sve opcionalno.
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

### Uobičajeni problemi

1. **401 greška autentifikacije**: Provjerite da ste postavili `x-api-key` zaglavlje na vaš HttpClient prije slanja DefaultAPI zahtjeva: `client.headers["x-api-key"] = "your-api-key"`
2. **Pogrešna API klasa**: Koristite `api_default` za server‑side autentifikovane zahtjeve, `api_public` za klijentske/javne zahtjeve i `api_moderation` za zahtjeve moderator‑dashboarda.