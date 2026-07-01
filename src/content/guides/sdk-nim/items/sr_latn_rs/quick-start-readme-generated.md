### Korišćenje Autentifikovanih API‑ja (DefaultAPI)

**Važno:** Autentifikovani krajnji poјaci zahtevaju da vaš API ključ bude postavljen kao `x-api-key` zaglavlje.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Napravite autentifikovane API pozive.
# Potrebni parametri (i telo zahteva) su pozicioni; opcioni
# parametri se prosleđuju putem objekta opcija operacije.
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

### Korišćenje javnih API‑ja (PublicAPI)

Javni krajnji poјaci ne zahtevaju autentifikaciju:

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

### Korišćenje Moderacionih API‑ja (ModerationAPI)

Moderacioni krajnji poјaci napajaju moderacijski kontrolni panel i autentifikovani su SSO tokenom za moderatora koji deluje:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Listajte komentare u moderacionom kontrolnom panelu.
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

1. **401 greška autentifikacije**: Proverite da ste postavili `x-api-key` zaglavlje na vaš HttpClient pre slanja DefaultAPI zahteva: `client.headers["x-api-key"] = "your-api-key"`
2. **Pogrešna API klasa**: Koristite `api_default` za serverske autentifikovane zahteve, `api_public` za klijentske/javne zahteve, i `api_moderation` za zahteve moderacionog kontrolnog panela.