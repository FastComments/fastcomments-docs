FastComments je avtorizacijski strežnik OAuth 2.1. Aplikacija lahko pridobi žeton, ki je vezan na en FastComments
račun, in ga uporablja na vseh končnih točkah v tem priročniku namesto API ključa. Na ta način se povezujejo Zapier aplikacija, strežnik MCP
in druge integracije tretjih oseb.

Žetoni se izdajajo prek avtorizacijskega toka s PKCE. Podpore za client credentials ali implicit grant ni.

### Odkrivanje

Lokacije končnih točk, podprti granti in avtentikacijske metode so objavljeni na standardnem URL-ju metapodatkov:

[inline-code-attrs-start title = 'Metapodatki avtorizacijskega strežnika'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Končne točke, ki jih opisuje:

[inline-code-attrs-start title = 'Končne točke OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Računi v regiji EU uporabljajo `https://eu.fastcomments.com` kot issuer, z istimi potmi.

### Registracija odjemalca

Odjemalec potrebuje `client_id` in registriran `redirect_uri`, preden lahko začne tok. Na voljo sta dva načina:

- **Dinamična registracija odjemalca.** `POST /oauth/register` z JSON telesom v skladu z RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Odgovor vsebuje `client_id` in, pri zaupnih odjemalcih, `client_secret`. Registracija je neavtenticirana in omejena s hitrostjo na IP.
- **Metadatni dokument ID odjemalca.** Odjemalec kot svoj `client_id` uporabi `https` URL, ki ga sam nadzoruje. FastComments pridobi ta URL in iz njega prebere ista metadatna polja. Klic za registracijo ni potreben.

Partnerske aplikacije, navedene v nadzorni plošči FastComments, kot je Zapier, registrira FastComments neposredno. Če gradite trgovinsko objavo in potrebujete odjemalca prve strani, se obrnite na podporo.

### Obsegi (scopes)

[inline-code-attrs-start title = 'Obsegi'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   zahtevki GET in HEAD
write  vse ostale metode
[inline-code-end]

Zahtevek, ki ne zahteva obsega, dobi oba. Uporabnik na strani za soglasje vidi zahtevane obsege. Zahtevek za obseg, ki ni eden od teh dveh, ne uspe z `invalid_scope`.

### Korak 1 - Avtorizacijski zahtevek

Brskalnik uporabnika pošljite na končno točko authorize. PKCE z metodo `S256` je obvezen za vsakega odjemalca.

[inline-code-attrs-start title = 'Avtorizacijski zahtevek'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parametri avtorizacijskega zahtevka'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Se mora točno ujemati z enim od registriranih preusmeritvenih URI odjemalca. **/
    redirect_uri: string
    /** Ločeni s presledkom. Izpustite za zahtevo obeh obsegov. **/
    scope?: 'read' | 'write' | 'read write'
    /** Vrnjen nespremenjen ob preusmeritvi. Uporabite ga za vezavo povratnega klica na sejo, ki je začela tok. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Neobvezen indikator vira po RFC 8707. Če je poslan, mora biti ista vrednost poslana tudi na končno točko token. **/
    resource?: string
}
[inline-code-end]

Uporabnik se po potrebi prijavi v FastComments in vidi stran za soglasje, ki navaja vašo aplikacijo, račun, na katerega bo povezana, in zahtevane obsege. Uporabnik mora imeti dovoljenje **API Admin** na tem računu; vsi drugi namesto obrazca za soglasje vidijo napako dovoljenja. Potrditev preusmeri brskalnik na vaš `redirect_uri` s `code` in `state`. Zavrnitev preusmeri z `error=access_denied`.

Avtorizacijska koda je veljavna 10 minut in jo je mogoče zamenjati enkrat. Druga zamenjava iste kode prekliče vse žetone, ki jih je ustvarila prva zamenjava.

### Korak 2 - Zahtevek za žeton

Zamenjajte kodo za žetone. Telo je kodirano kot obrazec. Zaupni odjemalci se avtenticirajo z `client_secret_basic` (HTTP Basic) ali `client_secret_post` (skrivnost v telesu). Javni odjemalci pošljejo samo `client_id`.

[inline-code-attrs-start title = 'Primer cURL zahtevka za žeton'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Telo zahtevka za žeton (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Samo zaupni odjemalci. Lahko je poslano namesto tega kot HTTP Basic avtentikacija. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Če je poslano, se mora ujemati z avtorizacijskim zahtevkom. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora z žetonom'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Predpona fcat_. Veljaven eno uro. **/
    access_token: string
    token_type: 'bearer'
    /** Sekunde do poteka dostopnega žetona. 3600. **/
    expires_in: number
    /** Predpona fcrt_. Veljaven 30 dni od izdaje. **/
    refresh_token: string
    /** Dodeljeni obsegi, ločeni s presledkom. **/
    scope: string
}
[inline-code-end]

Napake sledijo RFC 6749: JSON telo s `error` in `error_description`, HTTP 400 za `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` in `unsupported_grant_type`, HTTP 401 za `invalid_client`, HTTP 429 pri omejitvi hitrosti.

### Korak 3 - Klicanje API-ja

Dostopni žeton pošljite kot bearer žeton. Najemnik je implicitno določen z žetonom, zato je `tenantId` neobvezen. Če je podan, se mora ujemati z žetonom, sicer zahtevek ne uspe.

[inline-code-attrs-start title = 'Primer cURL z bearer žetonom'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` vrne najemnika, avtorizirajočega uporabnika in dodeljene obsege, zato je prava izbira za test povezave. Zahtevek s poteklim ali preklicanim žetonom dobi HTTP 401. Zahtevek, katerega metoda potrebuje obseg, ki ga žeton nima, dobi HTTP 403.

### Korak 4 - Osveževanje

[inline-code-attrs-start title = 'Primer cURL zahtevka za osvežitev'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Telo zahtevka za žeton (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Neobvezno. Zoži na podmnožico prvotno dodeljenih obsegov. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Odgovor ima isto obliko kot zamenjava kode. Osvežitveni žetoni se rotirajo: vsaka osvežitev vrne nov `refresh_token` in prekliče starega po 30-sekundnem obdobju milosti za sočasne zahtevke. Predstavitev osvežitvenega žetona, rotiranega pred več kot 30 sekundami, se obravnava kot ponovitev (replay) in prekliče celoten grant. Partnerske aplikacije, registrirane s strani FastComments, so izvzete iz rotacije in dobijo nazaj isti osvežitveni žeton z veljavnostjo, podaljšano za dodatnih 30 dni.

Osvežitev ponovno preveri, ali avtorizirajoči uporabnik še vedno ima API Admin na računu. Če ne, se grant prekliče in odgovor je `invalid_grant`.

### Preklic

[inline-code-attrs-start title = 'Primer cURL zahtevka za preklic'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Preklic osvežitvenega žetona prekliče vse dostopne žetone, izdane iz istega granta. Preklic dostopnega žetona prekliče samo ta žeton. Končna točka vrne HTTP 200 s praznim JSON objektom, ne glede na to, ali je bil žeton najden, v skladu z RFC 7009.

Uporabniki lahko povezavo prekličejo tudi v razdelku **Povezane aplikacije** v nadzorni plošči FastComments. Vsi žetoni za to aplikacijo takoj prenehajo delovati.