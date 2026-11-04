FastComments je OAuth 2.1 autorizacioni server. Aplikacija može da dobije token koji je vezan za jedan FastComments
nalog i da ga koristi na svakom endpointu u ovom vodiču umesto API ključa. Na ovaj way se povezuju Zapier aplikacija, MCP server
i druge integracije trećih strana.

Tokeni se izdaju kroz authorization code flow sa PKCE. Ne postoji client credentials niti implicit grant.

### Discovery

Lokacije endpointa, podržani grantovi i auth metode su objavljene na standardnom metadata URL-u:

[inline-code-attrs-start title = 'Metadata autorizacionog servera'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Endpointi koje on opisuje:

[inline-code-attrs-start title = 'OAuth endpointi'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Nalozi u EU regionu koriste `https://eu.fastcomments.com` kao issuer, sa istim putanjama.

### Registracija klijenta

Klijent treba `client_id` i registrovan `redirect_uri` pre nego što započne flow. Postoje dva načina da se to dobije:

- **Dinamička registracija klijenta (Dynamic Client Registration).** `POST /oauth/register` sa JSON telom prema RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Odgovor sadrži `client_id` i, za poverljive (confidential) klijente, `client_secret`. Registracija je neautentikovana i rate limited po IP adresi.
- **Client ID Metadata Document.** Klijent koristi `https` URL koji kontroliše kao svoj `client_id`. FastComments preuzima taj URL i čita ista metadata polja sa njega. Nije potreban poziv za registraciju.

Partnerske aplikacije navedene u FastComments dashboard-u, kao što je Zapier, registruje FastComments direktno. Kontaktirajte podršku ako pravite marketplace listing i treba vam first-party klijent.

### Scopes

[inline-code-attrs-start title = 'Scopes'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET i HEAD zahtevi
write  sve ostale metode
[inline-code-end]

Zahtev koji ne traži scope dobija oba. Korisnik vidi tražene scopes na stranici za saglasnost (consent). Zahtev za scope drugačiji od ova dva neuspeva sa `invalid_scope`.

### Korak 1 - Autorizacioni zahtev

Pošaljite korisnikov browser na authorize endpoint. PKCE sa metodom `S256` je obavezan za svaki klijent.

[inline-code-attrs-start title = 'Autorizacioni zahtev'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = 'Parametri autorizacionog zahteva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Must exactly match one of the client's registered redirect URIs. **/
    redirect_uri: string
    /** Space separated. Omit to request both scopes. **/
    scope?: 'read' | 'write' | 'read write'
    /** Returned unchanged on the redirect. Use it to bind the callback to the session that started the flow. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optional RFC 8707 resource indicator. If sent, the same value must be sent to the token endpoint. **/
    resource?: string
}
[inline-code-end]

Korisnik se prijavljuje na FastComments ako je potrebno i vidi stranicu za saglasnost koja navodi vašu aplikaciju, nalog na koji će biti povezana i tražene scopes. Korisnik mora imati dozvolu **API Admin** na tom nalogu; svi ostali umesto forme za saglasnost vide grešku o dozvolama. Odobravanje preusmerava browser na vaš `redirect_uri` sa `code` i `state`. Odbijanje preusmerava sa `error=access_denied`.

Autorizacioni kod je validan 10 minuta i može se zameniti jednom. Druga zamena istog koda opoziva sve tokene koje je prva zamena proizvela.

### Korak 2 - Zahtev za token

Zamenite kod za tokene. Telo je form encoded. Poverljivi (confidential) klijenti se autentikuju sa `client_secret_basic` (HTTP Basic) ili `client_secret_post` (tajna u telu). Javni klijenti šalju samo `client_id`.

[inline-code-attrs-start title = 'cURL primer zahteva za token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'Telo zahteva za token (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Confidential clients only. May be sent as HTTP Basic auth instead. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Must match the authorization request when sent. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora sa tokenom'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefixed fcat_. Valid for one hour. **/
    access_token: string
    token_type: 'bearer'
    /** Seconds until the access token expires. 3600. **/
    expires_in: number
    /** Prefixed fcrt_. Valid for 30 days from issue. **/
    refresh_token: string
    /** Space separated scopes granted. **/
    scope: string
}
[inline-code-end]

Greške prate RFC 6749: JSON telo sa `error` i `error_description`, HTTP 400 za `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` i `unsupported_grant_type`, HTTP 401 za `invalid_client`, HTTP 429 kada je rate limited.

### Korak 3 - Pozivanje API-ja

Pošaljite access token kao bearer token. Tenant je implikiran tokenom, pa je `tenantId` opcionalan. Ako je dat, mora se poklapati sa tokenom ili zahtev neuspeva.

[inline-code-attrs-start title = 'cURL primer sa bearer tokenom'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` vraća tenant, korisnika koji je autorizovao i dodeljene scopes, što ga čini pravim pozivom za test konekcije. Zahtev sa isteklim ili opozvanim tokenom dobija HTTP 401. Zahtev čija metoda zahteva scope koji token nema dobija HTTP 403.

### Korak 4 - Osvežavanje (refresh)

[inline-code-attrs-start title = 'cURL primer refresh zahteva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Telo zahteva za token (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Optional. Narrows to a subset of the scopes originally granted. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Odgovor ima isti oblik kao zamena koda. Refresh tokeni se rotiraju: svaki refresh vraća novi `refresh_token` i opoziva stari nakon 30-sekundnog grace prozora za konkurentne zahteve. Predstavljanje refresh tokena koji je rotiran pre više od 30 sekundi tretira se kao replay i opoziva ceo grant. Partnerske aplikacije registrovane od strane FastComments-a izuzete su od rotacije i dobijaju isti refresh token sa istekom produženim za još 30 dana.

Refresh takođe ponovo proverava da li autorizujući korisnik i dalje ima API Admin na nalogu. Ako nema, grant se opoziva i odgovor je `invalid_grant`.

### Opoziv (Revocation)

[inline-code-attrs-start title = 'cURL primer zahteva za opoziv'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Opoziv refresh tokena opoziva sve access tokene izdate iz istog granta. Opoziv access tokena opoziva samo taj token. Endpoint vraća HTTP 200 sa praznim JSON objektom bez obzira da li je token pronađen, prema RFC 7009.

Korisnici mogu da opozvu konekciju i iz **Connected Apps** u FastComments dashboard-u. Svi tokeni za tu aplikaciju odmah prestaju da rade.