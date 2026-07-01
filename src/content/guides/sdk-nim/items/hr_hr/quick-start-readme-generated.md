### Korištenje autentificiranih API‑ja (DefaultAPI)

**Važno:** Autentificirani krajnji točke zahtijevaju da vaš API ključ bude postavljen kao zaglavlje `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Napravite autentificirane API pozive.
# Potrebni parametri (i tijelo zahtjeva) su pozicijski; opcionalni
# parametri se prosljeđuju putem objekta opcija operacije.
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

### Korištenje javnih API‑ja (PublicAPI)

Javne krajnje točke ne zahtijevaju autentifikaciju:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Napravite javne API pozive.
# tenantId i urlId su zahtjevani (pozicijski); sve ostalo je opcionalno.
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

### Korištenje moderacijskih API‑ja (ModerationAPI)

Moderacijske krajnje točke napajaju moderacijsku nadzornu ploču i autentificirane su SSO tokenom za djelujućeg moderatora:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Popis komentara u moderacijskoj nadzornoj ploči.
# Ova operacija nema obavezne parametre, pa je sve opcionalno.
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

1. **401 greška autentifikacije**: Provjerite jeste li postavili zaglavlje `x-api-key` na vaš HttpClient prije slanja DefaultAPI zahtjeva: `client.headers["x-api-key"] = "your-api-key"`
2. **Pogrešna API klasa**: Koristite `api_default` za server‑side autentificirane zahtjeve, `api_public` za client‑side/javne zahtjeve i `api_moderation` za zahtjeve moderacijske nadzorne ploče.