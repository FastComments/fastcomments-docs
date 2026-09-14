FastComments je OAuth 2.1 poslužitelj autorizacije. Aplikacija može dobiti token koji je vezan uz jedan FastComments račun i koristiti ga na svakom krajnjem točku u ovom vodiču umjesto API ključa. Tako se Zapier aplikacija, MCP poslužitelj i druge integracije trećih strana povezuju.

Tokeni se izdaju putem toka autorizacijskog koda s PKCE-om. Ne postoji odobrenje klijentskih vjerodajnica niti implicitno odobrenje.

### Discovery

Lokacije krajnjih točaka, podržana odobrenja i metode autentifikacije objavljuju se na standardnoj URL adresi metapodataka:

[inline-code-attrs-start title = 'Metapodaci poslužitelja autorizacije'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Endpointi koje opisuje:

[inline-code-attrs-start title = 'OAuth krajnje točke'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Računi u EU regiji koriste `https://eu.fastcomments.com` kao izdavatelja, s istim putanjama.

### Registracija klijenta

Klijent treba `client_id` i registriranu `redirect_uri` prije nego što može započeti tok. Postoje dva načina za dobivanje:

- **Dinamička registracija klijenta.** `POST /oauth/register` s JSON tijelom prema RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Odgovor nosi `client_id` i, za povjerljive klijente, `client_secret`. Registracija je neautentificirana i ograničena po brzini po IP-u.
- **Dokument metapodataka ID klijenta.** Klijent koristi `https` URL koji kontrolira kao svoj `client_id`. FastComments dohvaća taj URL i čita iste metapodatke iz njega. Nije potreban poziv za registraciju.

Partner aplikacije navedene u FastComments nadzornoj ploči, poput Zapiera, registrira izravno FastComments. Kontaktirajte podršku ako izgradite popis na tržištu i trebate klijenta prve strane.

### Opsezi

[inline-code-attrs-start title = 'Opsezi'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Zahtjev koji ne traži opseg odobrava oba. Korisnik vidi tražene opsege na stranici pristanka. Zahtjev za opsegom koji nije jedan od ova dva ne uspijeva s `invalid_scope`.

### Korak 1 - Zahtjev za autorizaciju

Pošaljite preglednik korisnika na krajnju točku autorizacije. PKCE s metodom `S256` je obavezan za svakog klijenta.

[inline-code-attrs-start title = 'Zahtjev za autorizaciju'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parametri zahtjeva za autorizaciju'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Korisnik se prijavljuje u FastComments ako je potrebno i vidi stranicu pristanka koja navodi vašu aplikaciju, račun na koji će biti povezana i tražene opsege. Korisnik mora imati **API Admin** dozvolu na tom računu; bilo tko drugi vidi pogrešku dozvole umjesto obrasca pristanka. Odobrenje preusmjerava preglednik na vaš `redirect_uri` s `code` i `state`. Odbijanje preusmjerava s `error=access_denied`.

Autorizacijski kod je valjan 10 minuta i može se razmijeniti jednom. Druga razmjena istog koda opoziva svaki token koji je prva razmjena proizvela.

### Korak 2 - Zahtjev za token

Razmijenite kod za tokene. Tijelo je kodirano kao obrazac. Povjerljivi klijenti autentificiraju se s `client_secret_basic` (HTTP Basic) ili `client_secret_post` (tajna u tijelu). Javni klijenti šalju samo `client_id`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Tijelo zahtjeva za token (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora na token'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Pogreške slijede RFC 6749: JSON tijelo s `error` i `error_description`, HTTP 400 za `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` i `unsupported_grant_type`, HTTP 401 za `invalid_client`, HTTP 429 kada je ograničena brzina.

### Korak 3 - Pozivanje API-ja

Pošaljite pristupni token kao token nositelja. Najamnik je impliciran tokenom, pa je `tenantId` opcionalan. Kada je naveden, mora se podudarati s tokenom ili zahtjev ne uspijeva.

[inline-code-attrs-start title = 'Primjer cURL za token nositelja'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` vraća najamnika, ovlaštenog korisnika i odobrene opsege, što ga čini pravim pozivom za testiranje veze. Zahtjev s isteklim ili opozvanim tokenom dobiva HTTP 401. Zahtjev čija metoda zahtijeva opseg koji token ne posjeduje dobiva HTTP 403.

### Korak 4 - Osvježavanje

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za osvježavanje'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Tijelo zahtjeva za token (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Odgovor ima isti oblik kao razmjena koda. Osvježavajući tokeni rotiraju: svako osvježenje vraća novi `refresh_token` i opoziva stari nakon 30 sekundi grace perioda za istovremene zahtjeve. Predstavljanje osvježavajućeg tokena koji je rotiran prije više od 30 sekundi smatra se ponovnim korištenjem i opoziva cijelo odobrenje. Partner aplikacije registrirane od strane FastComments su izuzete od rotacije i vraćaju isti osvježavajući token s produženim rokom valjanosti za još 30 dana.

Osvježenje također provjerava da li ovlašteni korisnik i dalje ima API Admin na računu. Ako ne, odobrenje se opoziva i odgovor je `invalid_grant`.

### Opoziv

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za opoziv'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Opoziv osvježavajućeg tokena opoziva svaki pristupni token izdan iz istog odobrenja. Opoziv pristupnog tokena opoziva samo taj token. Krajnja točka vraća HTTP 200 s praznim JSON objektom, bez obzira je li token pronađen, prema RFC 7009.

Korisnici također mogu opozvati vezu iz **Povezanih aplikacija** u FastComments nadzornoj ploči. Svaki token za tu aplikaciju prestaje odmah raditi.