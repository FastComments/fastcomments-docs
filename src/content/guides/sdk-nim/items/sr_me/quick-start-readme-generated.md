### Korišćenje autentifikovanih API-ja (DefaultAPI)

**Važno:** Autentifikovani endpointi zahtijevaju da vaš API ključ bude postavljen kao `x-api-key` header.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Napravite autentifikovane API pozive.
# Obavezni parametri (i tijelo zahtjeva) su pozicioni; opcionalni
# parametri se prosljeđuju preko objekta opcija operacije.
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

### Korišćenje javnih API-ja (PublicAPI)

Javni endpointi ne zahtijevaju autentifikaciju:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Napravite javne API pozive.
# tenantId i urlId su obavezni (pozicioni); sve ostalo je opciono.
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

### Korišćenje moderacijskih API-ja (ModerationAPI)

Moderacijski endpointi pokreću moderatorsku kontrolnu tablu i autentifikovani su SSO tokenom za djelujućeg moderatora:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Listaj komentare u moderacijskoj kontrolnoj tabli.
# Ova operacija nema obavezne parametre, pa je sve opciono.
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

1. **401 greška autentifikacije**: Provjerite da ste postavili `x-api-key` header na vaš HttpClient prije slanja DefaultAPI zahjeva: `client.headers["x-api-key"] = "your-api-key"`
2. **Pogrešna API klasa**: Koristite `api_default` za server‑side autentifikovane zahtjeve, `api_public` za klijentske/javne zahtjeve i `api_moderation` za zahtjeve moderatorske kontrolne table.