FastComments is een OAuth 2.1 autorisatieserver. Een applicatie kan een token verkrijgen dat gekoppeld is aan één FastComments
account en dit op elk endpoint in deze handleiding gebruiken in plaats van een API-key. Zo verbinden de Zapier-app, de MCP-server
en andere integraties van derden zich.

Tokens worden verstrekt via de authorization code flow met PKCE. Er is geen client credentials of implicit grant.

### Discovery

Endpoint-locaties, ondersteunde grants en auth-methodes worden gepubliceerd op de standaard metadata-URL:

[inline-code-attrs-start title = 'Autorisatieserver Metadata'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

De endpoints die het beschrijft:

[inline-code-attrs-start title = 'OAuth Endpoints'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Accounts in de EU-regio gebruiken `https://eu.fastcomments.com` als issuer, met dezelfde paden.

### Een client registreren

Een client heeft een `client_id` en een geregistreerde `redirect_uri` nodig voordat de flow kan starten. Er zijn twee manieren om die te verkrijgen:

- **Dynamic Client Registration.** `POST /oauth/register` met een JSON-body volgens RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Het antwoord bevat de `client_id` en, voor confidential clients, de `client_secret`. Registratie vindt zonder authenticatie plaats en is rate limited per IP.
- **Client ID Metadata Document.** De client gebruikt een `https`-URL die het zelf beheert als `client_id`. FastComments haalt die URL op en leest dezelfde metadata-velden eruit. Er is geen registratie-aanroep nodig.

Partnerapplicaties die in het FastComments-dashboard staan vermeld, zoals Zapier, worden door FastComments zelf geregistreerd. Neem contact op met support als je een marketplace-vermelding bouwt en een first-party client nodig hebt.

### Scopes

[inline-code-attrs-start title = 'Scopes'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Een verzoek zonder scope krijgt beide. De gebruiker ziet de aangevraagde scopes op de toestemmingspagina. Een verzoek om een andere scope dan deze twee mislukt met `invalid_scope`.

### Stap 1 - Autorisatieverzoek

Stuur de browser van de gebruiker naar het authorize-endpoint. PKCE met de `S256`-methode is verplicht voor elke client.

[inline-code-attrs-start title = 'Autorisatieverzoek'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parameters van het Autorisatieverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

De gebruiker logt zo nodig in bij FastComments en ziet een toestemmingspagina met de naam van je applicatie, het account waarmee deze wordt verbonden en de aangevraagde scopes. De gebruiker moet de **API Admin**-rechten hebben op dat account; iedereen anders ziet een rechtenfout in plaats van het toestemmingsformulier. Goedkeuren stuurt de browser door naar je `redirect_uri` met `code` en `state`. Weigeren stuurt door met `error=access_denied`.

De autorisatiecode is 10 minuten geldig en kan één keer worden ingewisseld. Een tweede inwisseling van dezelfde code trekt alle tokens in die de eerste inwisseling heeft opgeleverd.

### Stap 2 - Tokenverzoek

Wissel de code in voor tokens. De body is form-encoded. Confidential clients authenticeren met `client_secret_basic` (HTTP Basic) of `client_secret_post` (secret in de body). Public clients sturen alleen `client_id`.

[inline-code-attrs-start title = 'Tokenverzoek cURL-voorbeeld'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Body van het Tokenverzoek (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structuur van het Tokenantwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Fouten volgen RFC 6749: een JSON-body met `error` en `error_description`, HTTP 400 voor `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` en `unsupported_grant_type`, HTTP 401 voor `invalid_client`, HTTP 429 bij rate limiting.

### Stap 3 - De API aanroepen

Stuur het access token mee als bearer token. De tenant volgt impliciet uit het token, dus `tenantId` is optioneel. Als deze wordt meegegeven, moet deze overeenkomen met het token, anders mislukt het verzoek.

[inline-code-attrs-start title = 'Bearer Token cURL-voorbeeld'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` geeft de tenant, de autoriserende gebruiker en de verleende scopes terug, waardoor dit de juiste aanroep is voor een verbindingstest. Een verzoek met een verlopen of ingetrokken token krijgt HTTP 401. Een verzoek waarvan de methode een scope nodig heeft die het token niet heeft, krijgt HTTP 403.

### Stap 4 - Vernieuwen

[inline-code-attrs-start title = 'Vernieuwingsverzoek cURL-voorbeeld'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Body van het Tokenverzoek (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Het antwoord heeft dezelfde vorm als de code-inwisseling. Refresh tokens roteren: elke vernieuwing geeft een nieuwe `refresh_token` terug en trekt de oude in na een respijtvenster van 30 seconden voor gelijktijdige verzoeken. Het aanbieden van een refresh token dat meer dan 30 seconden geleden is geroteerd, wordt behandeld als replay en trekt de hele grant in. Partnerapplicaties die door FastComments zijn geregistreerd, zijn uitgezonderd van rotatie en krijgen dezelfde refresh token terug met een verlengde geldigheid van nog eens 30 dagen.

Bij een vernieuwing wordt ook opnieuw gecontroleerd of de autoriserende gebruiker nog steeds API Admin heeft op het account. Zo niet, dan wordt de grant ingetrokken en is het antwoord `invalid_grant`.

### Intrekken

[inline-code-attrs-start title = 'Intrekkingsverzoek cURL-voorbeeld'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Het intrekken van een refresh token trekt alle access tokens in die vanuit dezelfde grant zijn verstrekt. Het intrekken van een access token trekt alleen dat token in. Het endpoint geeft HTTP 200 terug met een leeg JSON-object, ongeacht of het token is gevonden, conform RFC 7009.

Gebruikers kunnen een verbinding ook intrekken via **Connected Apps** in het FastComments-dashboard. Elke token voor die applicatie stopt onmiddellijk met werken.