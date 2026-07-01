### Uporaba avtenticiranih API-jev (DefaultAPI)

**Pomembno:** Avtenticirani končni naslovi zahtevajo, da je vaš API ključ nastavljen v glavi `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Naredi avtenticirane klice API-ja.
# Zahtevani parametri (in telo zahteve) so položajni; neobvezni
# parametri se posredujejo prek objekt options operacije.
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

### Uporaba javnih API-jev (PublicAPI)

Javni končni naslovi ne zahtevajo avtentikacije:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Naredi javne klice API-ja.
# tenantId in urlId sta obvezna (položajna); vse ostalo je neobvezno.
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

### Uporaba moderacijskih API-jev (ModerationAPI)

Moderacijski končni naslovi poganjajo nadzorno ploščo moderatorja in so avtenticirani z SSO žetonom za delujočega moderatorja:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Izpiši komentarje v moderacijski nadzorni plošči.
# Ta operacija nima zahtevanih parametrov, zato je vse neobvezno.
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

### Pogoste težave

1. **Napaka 401 pri avtentikaciji**: Prepričajte se, da ste nastavili glavo `x-api-key` na vašem HttpClient-u pred pošiljanjem zahtevanj DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Napačen API razred**: Uporabite `api_default` za strežniške avtenticirane zahteve, `api_public` za odjemalske/javne zahteve in `api_moderation` za zahteve moderacijske nadzorne plošče.