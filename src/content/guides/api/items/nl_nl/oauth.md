FastComments is een OAuth 2.1 autorisatieserver. Een applicatie kan een token verkrijgen dat is gekoppeld aan één FastComments‑account en dit gebruiken op elk eindpunt in deze gids in plaats van een API‑sleutel. Zo verbinden de Zapier‑app, de MCP‑server en andere integraties van derden.

Tokens worden uitgegeven via de autorisatiecode‑flow met PKCE. Er is geen client‑credentials‑ of impliciete grant.

### Discovery

Endpointlocaties, ondersteunde grants en authenticatiemethoden worden gepubliceerd op de standaard metadata‑URL:

[inline-code-attrs-start title = 'Metadata van autorisatieserver'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

De eindpunten die het beschrijft:

[inline-code-attrs-start title = 'OAuth-eindpunten'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Accounts in de EU‑regio gebruiken `https://eu.fastcomments.com` als issuer, met dezelfde paden.

### Registering a client

Een client heeft een `client_id` en een geregistreerde `redirect_uri` nodig voordat hij de flow kan starten. Er zijn twee manieren om er één te krijgen:

- **Dynamische clientregistratie.** `POST /oauth/register` met een JSON‑body volgens RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Het antwoord bevat de `client_id` en, voor vertrouwelijke clients, de `client_secret`. Registratie is niet‑geauthenticeerd en per IP rate‑gelimiteerd.
- **Client‑ID‑metadata‑document.** De client gebruikt een `https`‑URL die hij beheert als zijn `client_id`. FastComments haalt die URL op en leest dezelfde metadata‑velden ervan. Een registratie‑aanroep is niet nodig.

Partnerapplicaties die in het FastComments‑dashboard staan, zoals Zapier, worden rechtstreeks door FastComments geregistreerd. Neem contact op met de support als je een marktplaatsvermelding bouwt en een first‑party client nodig hebt.

### Scopes

[inline-code-attrs-start title = 'Scopes'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Een verzoek dat geen scope vraagt, krijgt beide toegekend. De gebruiker ziet de gevraagde scopes op de toestemmingspagina. Een verzoek voor een andere scope dan deze twee mislukt met `invalid_scope`.

### Stap 1 – Autorisatie‑verzoek

Stuur de browser van de gebruiker naar het autorisatie‑eindpunt. PKCE met de `S256`‑methode is vereist voor elke client.

[inline-code-attrs-start title = 'Autorisatie‑verzoek'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parameters van autorisatie‑verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Moet exact overeenkomen met een van de geregistreerde redirect‑URI's van de client. **/
    redirect_uri: string
    /** Spatie‑gescheiden. Laat weg om beide scopes aan te vragen. **/
    scope?: 'read' | 'write' | 'read write'
    /** Wordt ongewijzigd teruggegeven bij de redirect. Gebruik het om de callback te binden aan de sessie die de flow startte. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optionele RFC 8707 resource‑indicator. Indien verzonden, moet dezelfde waarde naar het token‑eindpunt worden gestuurd. **/
    resource?: string
}
[inline-code-end]

De gebruiker logt in op FastComments indien nodig en ziet een toestemmingspagina met de naam van je applicatie, het account waarmee deze wordt verbonden, en de gevraagde scopes. De gebruiker moet de **API Admin**‑permissie op dat account hebben; anderen zien een permissiefout in plaats van het toestemmingsformulier. Goedkeuren leidt tot een redirect van de browser naar je `redirect_uri` met `code` en `state`. Afwijzen leidt tot een redirect met `error=access_denied`.

De autorisatiecode is 10 minuten geldig en kan één keer worden ingewisseld. Een tweede uitwisseling van dezelfde code intrekt elk token dat de eerste uitwisseling heeft opgeleverd.

### Stap 2 – Token‑verzoek

Wissel de code in voor tokens. De body is form‑gecodeerd. Vertrouwelijke clients authenticeren met `client_secret_basic` (HTTP Basic) of `client_secret_post` (geheim in de body). Publieke clients sturen alleen `client_id`.

[inline-code-attrs-start title = 'Voorbeeld cURL‑verzoek token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Token‑verzoek body (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Alleen voor vertrouwelijke clients. Kan in plaats daarvan als HTTP Basic‑authenticatie worden verzonden. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Moet overeenkomen met het autorisatie‑verzoek wanneer verzonden. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur token‑respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Voorafgegaan door fcat_. Geldig voor één uur. **/
    access_token: string
    token_type: 'bearer'
    /** Seconden tot het access‑token verloopt. 3600. **/
    expires_in: number
    /** Voorafgegaan door fcrt_. Geldig voor 30 dagen vanaf uitgifte. **/
    refresh_token: string
    /** Spatie‑gescheiden toegekende scopes. **/
    scope: string
}
[inline-code-end]

Fouten volgen RFC 6749: een JSON‑body met `error` en `error_description`, HTTP 400 voor `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` en `unsupported_grant_type`, HTTP 401 voor `invalid_client`, HTTP 429 bij rate‑limiting.

### Stap 3 – API‑aanroep

Stuur het access‑token als een bearer‑token. De tenant wordt afgeleid van het token, dus `tenantId` is optioneel. Indien opgegeven moet het overeenkomen met het token, anders faalt het verzoek.

[inline-code-attrs-start title = 'Voorbeeld cURL‑verzoek bearer‑token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` retourneert de tenant, de autoriserende gebruiker en de toegekende scopes, waardoor het de juiste oproep is voor een verbindingstest. Een verzoek met een verlopen of ingetrokken token krijgt HTTP 401. Een verzoek waarvan de methode een scope vereist die het token niet bezit, krijgt HTTP 403.

### Stap 4 – Vernieuwing

[inline-code-attrs-start title = 'Voorbeeld cURL‑verzoek vernieuwing'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Token‑verzoek body (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Optioneel. Beperkt tot een subset van de oorspronkelijk toegekende scopes. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Het antwoord heeft dezelfde structuur als de code‑uitwisseling. Refresh‑tokens roteren: elke vernieuwing geeft een nieuw `refresh_token` terug en intrekt het oude na een wachttijd van 30 seconden voor gelijktijdige verzoeken. Het presenteren van een refresh‑token dat meer dan 30 seconden geleden is geroteerd, wordt behandeld als replay en intrekt de volledige grant. Partnerapplicaties die door FastComments zijn geregistreerd, zijn vrijgesteld van rotatie en krijgen hetzelfde refresh‑token terug met een verlengde vervaldatum van nog eens 30 dagen.

Een vernieuwing controleert ook opnieuw of de autoriserende gebruiker nog steeds API Admin op het account heeft. Zo niet, dan wordt de grant ingetrokken en is het antwoord `invalid_grant`.

### Intrekking

[inline-code-attrs-start title = 'Voorbeeld cURL‑verzoek intrekking'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Het intrekken van een refresh‑token intrekt elk access‑token dat uit dezelfde grant is uitgegeven. Het intrekken van een access‑token intrekt alleen dat token. Het eindpunt retourneert HTTP 200 met een leeg JSON‑object, ongeacht of het token werd gevonden, volgens RFC 7009.

Gebruikers kunnen ook een verbinding intrekken via **Connected Apps** in het FastComments‑dashboard. Elk token voor die applicatie stopt onmiddellijk met werken.