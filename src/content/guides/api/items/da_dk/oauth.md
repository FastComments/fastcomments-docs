FastComments er en OAuth 2.1 autorisationsserver. En applikation kan hente et token, der er bundet til en FastComments‑konto, og bruge det på alle endpoint i denne vejledning i stedet for en API‑nøgle. Sådan forbinder Zapier‑appen, MCP‑serveren og andre tredjepartsintegrationer.

Tokens udstedes via autorisationskodeflowet med PKCE. Der findes ingen klientlegitimations‑ eller implicit tilladelse.

### Opdagelse

Endpoint‑placeringer, understøttede tilladelser og autentificeringsmetoder offentliggøres på den standard metadata‑URL:

[inline-code-attrs-start title = 'Autoriseringsserver Metadata'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

De endpoint, den beskriver:

[inline-code-attrs-start title = 'OAuth‑endpoints'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Kontoer i EU‑regionen bruger `https://eu.fastcomments.com` som udsteder, med de samme stier.

### Registrering af en klient

En klient har brug for et `client_id` og en registreret `redirect_uri`, før den kan starte flowet. Der er to måder at få en på:

- **Dynamisk klientregistrering.** `POST /oauth/register` med en JSON‑krop i henhold til RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Svaret indeholder `client_id` og, for fortrolige klienter, `client_secret`. Registreringen er uautentificeret og hastighedsbegrænset pr. IP.
- **Klient‑ID‑metadata‑dokument.** Klienten bruger en `https`‑URL, den kontrollerer, som sit `client_id`. FastComments henter den URL og læser de samme metadatafelter fra den. Der er ikke behov for et registreringskald.

Partner‑applikationer, der er listet i FastComments‑dashboardet, såsom Zapier, registreres direkte af FastComments. Kontakt support, hvis du bygger en markedsplads‑liste og har brug for en første‑part‑klient.

### Omfang

[inline-code-attrs-start title = 'Omfang'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

En anmodning, der ikke anmoder om noget omfang, får begge tildelt. Brugeren ser de anmodede omfang på samtykkesiden. En anmodning om et omfang, der ikke er et af de to, fejler med `invalid_scope`.

### Trin 1 – Autoriseringsanmodning

Send brugerens browser til autoriserings‑endpointet. PKCE med `S256`‑metoden er påkrævet for hver klient.

[inline-code-attrs-start title = 'Autoriseringsanmodning'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Autoriseringsanmodningsparametre'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Brugeren logger ind på FastComments om nødvendigt og ser en samtykkeside, der navngiver din applikation, den konto den vil blive forbundet til, og de anmodede omfang. Brugeren skal have **API‑Admin**‑tilladelsen på den konto; alle andre ser en tilladelsesfejl i stedet for samtykkesiden. Godkendelse omdirigerer browseren til din `redirect_uri` med `code` og `state`. Afvisning omdirigerer med `error=access_denied`.

Autorisationstokenet er gyldigt i 10 minutter og kan udveksles én gang. En anden udveksling af den samme kode tilbagekalder alle tokens, som den første udveksling producerede.

### Trin 2 – Token‑anmodning

Udveksl koden for tokens. Kroppen er formular‑kodet. Fortrolige klienter autentificerer med `client_secret_basic` (HTTP Basic) eller `client_secret_post` (hemmelighed i kroppen). Offentlige klienter sender kun `client_id`.

[inline-code-attrs-start title = 'Token‑anmodning cURL‑eksempel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Token‑anmodningskrop (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Token‑responsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Fejl følger RFC 6749: en JSON‑krop med `error` og `error_description`, HTTP 400 for `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` og `unsupported_grant_type`, HTTP 401 for `invalid_client`, HTTP 429 ved hastighedsbegrænsning.

### Trin 3 – Kald af API'en

Send adgangstokenet som en bearer‑token. Lejeren er implikeret af tokenet, så `tenantId` er valgfri. Når den er angivet, skal den matche tokenet, ellers fejler anmodningen.

[inline-code-attrs-start title = 'Bearer‑token cURL‑eksempel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` returnerer lejeren, den autoriserende bruger og de tildelte omfang, hvilket gør den til det rette kald for en forbindelsestest. En anmodning med et udløbet eller tilbagekaldt token får HTTP 401. En anmodning, hvis metode kræver et omfang, som tokenet ikke har, får HTTP 403.

### Trin 4 – Opfriskning

[inline-code-attrs-start title = 'Opfrisknings‑anmodning cURL‑eksempel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Token‑anmodningskrop (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Svaret har samme struktur som kode‑udvekslingen. Opfriskningstokens roteres: hver opfriskning returnerer et nyt `refresh_token` og tilbagekalder det gamle efter et 30‑sekunders grace‑vindue for samtidige anmodninger. Præsentation af et opfriskningstoken, der blev roteret for mere end 30 sekunder siden, betragtes som replay og tilbagekalder hele tilladelsen. Partner‑applikationer, der er registreret af FastComments, er undtaget fra rotation og får det samme opfriskningstoken med udløbet forlænget med yderligere 30 dage.

En opfriskning tjekker også, at den autoriserende bruger stadig har API‑Admin på kontoen. Hvis ikke, tilbagekaldes tilladelsen, og svaret er `invalid_grant`.

### Tilbagekaldelse

[inline-code-attrs-start title = 'Tilbagekaldelses‑anmodning cURL‑eksempel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Tilbagekaldelse af et opfriskningstoken tilbagekalder alle adgangstokens udstedt fra den samme tilladelse. Tilbagekaldelse af et adgangstoken tilbagekalder kun det pågældende token. Endpointet returnerer HTTP 200 med et tomt JSON‑objekt, uanset om tokenet blev fundet, i henhold til RFC 7009.

Brugere kan også tilbagekalde en forbindelse fra **Connected Apps** i FastComments‑dashboardet. Alle tokens for den pågældende applikation stopper med at fungere med det samme.