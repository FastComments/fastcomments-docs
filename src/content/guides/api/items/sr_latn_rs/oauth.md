FastComments je OAuth 2.1 server za autorizaciju. Aplikacija može dobiti token koji je vezan za jedan FastComments nalog i koristiti ga na svakom krajnjem punktu u ovom vodiču umesto API ključa. Tako se Zapier aplikacija, MCP server i druge integracije trećih strana povezuju.

Tokeni se izdaju kroz tok autorizacionog koda sa PKCE. Ne postoji grant za klijentske kredencijale niti implicitni grant.

### Discovery

Lokacije krajnjih tačaka, podržani grantovi i metode autentifikacije objavljuju se na standardnoj URL adresi metapodataka:

[inline-code-attrs-start title = 'Metapodaci servera autorizacije'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Krajnje tačke koje opisuje:

[inline-code-attrs-start title = 'OAuth krajnje tačke'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Nalozi u EU regionu koriste `https://eu.fastcomments.com` kao izdavača, sa istim putanjama.

### Registering a client

Klijent treba `client_id` i registrovani `redirect_uri` pre nego što može započeti tok. Postoje dva načina da se dobije:

- **Dinamička registracija klijenta.** `POST /oauth/register` sa JSON telom prema RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Odgovor nosi `client_id` i, za poverljive klijente, `client_secret`. Registracija nije autentifikovana i ograničena je po IP adresi.
- **Dokument metapodataka ID klijenta.** Klijent koristi `https` URL koji kontroliše kao svoj `client_id`. FastComments preuzima taj URL i čita iste metapodatke iz njega. Nije potreban poziv za registraciju.

Partnerske aplikacije navedene u FastComments kontrolnoj tabli, poput Zapiera, registruje direktno FastComments. Kontaktirajte podršku ako pravite listing na tržištu i potrebna vam je klijent aplikacija prve strane.

### Scopes

[inline-code-attrs-start title = 'Opsezi'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Zahtev koji ne traži nikakav opseg dobija oba. Korisnik vidi tražene opsege na stranici saglasnosti. Zahtev za opseg koji nije jedan od ova dva ne uspeva i vraća `invalid_scope`.

### Step 1 - Authorization request

Pošaljite pregledač korisnika na endpoint za autorizaciju. PKCE sa metodom `S256` je obavezan za svakog klijenta.

[inline-code-attrs-start title = 'Zahtev za autorizaciju'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parametri zahteva za autorizaciju'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Mora tačno da se podudara sa jednim od registrovanih redirect URI-ja klijenta. **/
    redirect_uri: string
    /** Razdvojeni razmacima. Omitujte da biste zatražili oba opsega. **/
    scope?: 'read' | 'write' | 'read write'
    /** Vraća se nepromenjen na preusmeravanju. Koristite ga da povežete povratni poziv sa sesijom koja je započela tok. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Opcioni RFC 8707 indikator resursa. Ako se pošalje, ista vrednost mora biti poslata i na token endpoint. **/
    resource?: string
}
[inline-code-end]

Korisnik se prijavljuje na FastComments ako je potrebno i vidi stranicu saglasnosti koja navodi vašu aplikaciju, nalog na koji će biti povezana i tražene opsege. Korisnik mora imati **API Admin** dozvolu na tom nalogu; bilo ko drugi vidi grešku dozvole umesto obrasca za saglasnost. Odobravanje preusmerava pregledač na vaš `redirect_uri` sa `code` i `state`. Odbijanje preusmerava sa `error=access_denied`.

Autorizacioni kod je važeći 10 minuta i može se razmeniti jednom. Druga razmena istog koda opoziva sve tokene koje je prva razmena proizvela.

### Step 2 - Token request

Zamenite kod za tokene. Telo je kodirano kao formular. Poverljivi klijenti se autentifikuju sa `client_secret_basic` (HTTP Basic) ili `client_secret_post` (tajna u telu). Javni klijenti šalju samo `client_id`.

[inline-code-attrs-start title = 'Primer cURL zahteva za token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
    /** Samo poverljivi klijenti. Može se poslati kao HTTP Basic autentifikacija umesto. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Mora se podudarati sa zahtevom za autorizaciju kada se pošalje. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za token'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefiks fcat_. Važi jedan sat. **/
    access_token: string
    token_type: 'bearer'
    /** Sekunde do isteka pristupnog tokena. 3600. **/
    expires_in: number
    /** Prefiks fcrt_. Važi 30 dana od izdavanja. **/
    refresh_token: string
    /** Razdvojeni razmacima dodeljeni opsezi. **/
    scope: string
}
[inline-code-end]

Greške prate RFC 6749: JSON telo sa `error` i `error_description`, HTTP 400 za `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` i `unsupported_grant_type`, HTTP 401 za `invalid_client`, HTTP 429 kada je ograničeno po stopi.

### Step 3 - Calling the API

Pošaljite pristupni token kao bearer token. Tenant je impliciran tokenom, pa je `tenantId` opcionalan. Kada se navede, mora se podudarati sa tokenom ili zahtev ne uspeva.

[inline-code-attrs-start title = 'Primer cURL zahteva za Bearer token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` vraća tenant, korisnika koji je autorizovao i dodeljene opsege, što ga čini pravim pozivom za testiranje veze. Zahtev sa isteklim ili opozvanim tokenom dobija HTTP 401. Zahtev čiji metod zahteva opseg koji token ne poseduje dobija HTTP 403.

### Step 4 - Refreshing

[inline-code-attrs-start title = 'Primer cURL zahteva za osvežavanje'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
    /** Opcionalno. Sužava na podskup originalno dodeljenih opsega. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Odgovor ima isti oblik kao razmena koda. Refresh tokeni se rotiraju: svaki osvežavanje vraća novi `refresh_token` i opoziva stari nakon 30 sekundi perioda odlaganja za paralelne zahteve. Predstavljanje refresh tokena koji je rotiran pre više od 30 sekundi tretira se kao ponovni pokušaj i opoziva celokupni grant. Partnerske aplikacije registrovane od strane FastComments su izuzete od rotacije i dobijaju isti refresh token uz produženo vreme isteka za još 30 dana.

Osvežavanje takođe proverava da li korisnik koji je autorizovao i dalje ima API Admin dozvolu na nalogu. Ako ne, grant se opoziva i odgovor je `invalid_grant`.

### Revocation

[inline-code-attrs-start title = 'Primer cURL zahteva za opoziv'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Opoziv refresh tokena opoziva sve pristupne tokene izdane iz istog granta. Opoziv pristupnog tokena opoziva samo taj token. Endpoint vraća HTTP 200 sa praznim JSON objektom bez obzira da li je token pronađen, prema RFC 7009.

Korisnici takođe mogu opozvati vezu iz **Connected Apps** u FastComments kontrolnoj tabli. Svaki token za tu aplikaciju prestaje da radi odmah.