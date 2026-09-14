FastComments je strežnik za avtorizacijo OAuth 2.1. Aplikacija lahko pridobi žeton, ki je vezan na en račun FastComments, in ga uporabi na vseh končnih točkah v tem vodniku namesto API ključa. Tako se povezuje aplikacija Zapier, strežnik MCP in druge integracije tretjih strani.

Žetoni se izdajajo prek toka avtorizacijskega kode s PKCE. Ni podpore za odobritev s poverilnicami odjemalca ali implicitno odobritev.

### Discovery

Lokacije končnih točk, podprte odobritve in metode avtorizacije so objavljene na standardnem URL-ju metapodatkov:

[inline-code-attrs-start title = 'Metapodatki strežnika za avtorizacijo'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Končne točke, ki jih opisuje:

[inline-code-attrs-start title = 'OAuth končne točke'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Računi v regiji EU uporabljajo `https://eu.fastcomments.com` kot izdajatelja, z enakimi potmi.

### Registering a client

Odjemalec potrebuje `client_id` in registriran `redirect_uri`, preden lahko začne tok. Obstajata dva načina, kako ga pridobiti:

- **Dinamična registracija odjemalca.** `POST /oauth/register` z JSON telo po RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Odgovor vsebuje `client_id` in, za zaupne odjemalce, `client_secret`. Registracija je neavtenticirana in omejena po hitrosti na IP.
- **Dokument metapodatkov ID odjemalca.** Odjemalec uporablja `https` URL, ki ga nadzoruje, kot svoj `client_id`. FastComments pridobi ta URL in prebere iste metapodatke iz njega. Klic za registracijo ni potreben.

Partnerske aplikacije, navedene v nadzorni plošči FastComments, kot je Zapier, registrira neposredno FastComments. Obrnite se na podporo, če gradite seznam na tržnici in potrebujete odjemalca prve strani.

### Scopes

[inline-code-attrs-start title = 'Obsegi'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Zahteva, ki ne zahteva nobenega obsega, prejme oba. Uporabnik vidi zahtevane obsege na strani za soglasje. Zahteva za obseg, ki ni eden od teh dveh, spodleti z `invalid_scope`.

### Step 1 - Authorization request

Pošljite brskalnik uporabnika na končno točko za avtorizacijo. PKCE z metodo `S256` je zahtevan za vsakega odjemalca.

[inline-code-attrs-start title = 'Zahteva za avtorizacijo'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parametri zahteve za avtorizacijo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Mora se natančno ujemati z enim od registriranih preusmeritvenih URI-jev odjemalca. **/
    redirect_uri: string
    /** Ločeno s presledki. Izpustite, da zahtevate oba obsega. **/
    scope?: 'read' | 'write' | 'read write'
    /** Vrnjeno nespremenjeno na preusmeritvi. Uporabite ga za vezavo povratnega klica na sejo, ki je začela tok. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Neobvezen indikator vira po RFC 8707. Če je poslan, mora ista vrednost biti poslana tudi na končno točko za žeton. **/
    resource?: string
}
[inline-code-end]

Uporabnik se po potrebi prijavi v FastComments in vidi stran za soglasje, ki navaja vašo aplikacijo, račun, s katerim bo povezana, in zahtevane obsege. Uporabnik mora imeti dovoljenje **API Admin** na tem računu; vsi drugi vidijo napako dovoljenja namesto obrazca za soglasje. Odobritev preusmeri brskalnik na vaš `redirect_uri` z `code` in `state`. Zavrnjena zahteva preusmeri z `error=access_denied`.

Avtorizacijska koda je veljavna 10 minut in se lahko izmenja enkrat. Druga izmenjava iste kode prekliče vsak žeton, ki ga je ustvarila prva izmenjava.

### Step 2 - Token request

Izmenjajte kodo za žetone. Telo je kodirano kot obrazec. Zaupni odjemalci se avtenticirajo z `client_secret_basic` (HTTP Basic) ali `client_secret_post` (skrivnost v telesu). Javni odjemalci pošljejo le `client_id`.

[inline-code-attrs-start title = 'Primer cURL zahteve za žeton'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Telo zahteve za žeton (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Samo za zaupne odjemalce. Lahko se pošlje tudi kot HTTP Basic avtentikacija. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Mora se ujemati z avtorizacijsko zahtevo, ko je poslan. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora žetona'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefiks fcat_. Veljaven eno uro. **/
    access_token: string
    token_type: 'bearer'
    /** Sekunde do poteka dostopnega žetona. 3600. **/
    expires_in: number
    /** Prefiks fcrt_. Veljaven 30 dni od izdaje. **/
    refresh_token: string
    /** Ločeno s presledki odobreni obsegi. **/
    scope: string
}
[inline-code-end]

Napake sledijo RFC 6749: JSON telo z `error` in `error_description`, HTTP 400 za `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` in `unsupported_grant_type`, HTTP 401 za `invalid_client`, HTTP 429 pri omejitvi hitrosti.

### Step 3 - Calling the API

Pošljite dostopni žeton kot žeton nosilec. Najemnik je impliciran v žetonu, zato je `tenantId` neobvezen. Če je podan, se mora ujemati z žetonom, sicer zahteva spodleti.

[inline-code-attrs-start title = 'Primer cURL žetona nosilca'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` vrne najemnika, avtenticiranega uporabnika in odobrene obsege, kar ga naredi za pravilen klic za test povezave. Zahteva s poteklim ali preklicanim žetonom prejme HTTP 401. Zahteva, katere metoda zahteva obseg, ki ga žeton ne vsebuje, prejme HTTP 403.

### Step 4 - Refreshing

[inline-code-attrs-start title = 'Primer cURL zahteve za osvežitev'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Telo zahteve za žeton (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Neobvezno. Omeji na podskupino izvirno odobrenih obsegov. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Odgovor ima enako strukturo kot izmenjava kode. Osvežitveni žetoni se rotirajo: vsako osvežitev vrne nov `refresh_token` in prekliče prejšnjega po 30-sekundnem obdobju odlogu za sočasne zahteve. Predložitev osvežitvenega žetona, ki je bil rotiran pred več kot 30 sekundami, se obravnava kot ponovna uporaba in prekliče celotno odobritev. Partnerske aplikacije, registrirane s strani FastComments, so izvzete iz rotacije in prejmejo isti osvežitveni žeton z podaljšanim rokom veljavnosti za dodatnih 30 dni.

Osvežitev tudi ponovno preveri, ali avtenticirani uporabnik še vedno ima dovoljenje API Admin na računu. Če ne, se odobritev prekliče in odgovor je `invalid_grant`.

### Revocation

[inline-code-attrs-start title = 'Primer cURL zahteve za preklic'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Preklic osvežitvenega žetona prekliče vsak dostopni žeton, izdan iz iste odobritve. Preklic dostopnega žetona prekliče le ta žeton. Končna točka vrne HTTP 200 s praznim JSON objektom, ne glede na to, ali je bil žeton najden, po RFC 7009.

Uporabniki lahko tudi prekličejo povezavo iz **Povezanih aplikacij** v nadzorni plošči FastComments. Vsak žeton za to aplikacijo takoj preneha delovati.