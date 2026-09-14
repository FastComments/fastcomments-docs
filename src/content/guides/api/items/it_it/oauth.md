FastComments è un server di autorizzazione OAuth 2.1. Un'applicazione può ottenere un token associato a un account FastComments e usarlo su ogni endpoint in questa guida al posto di una chiave API. È così che l'app Zapier, il server MCP e altre integrazioni di terze parti si connettono.

I token vengono emessi tramite il flusso di codice di autorizzazione con PKCE. Non esistono grant di credenziali client né grant impliciti.

### Scoperta

Le posizioni degli endpoint, i grant supportati e i metodi di autenticazione sono pubblicati all'URL standard dei metadati:

[inline-code-attrs-start title = 'Metadata del Server di Autorizzazione'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Gli endpoint che descrive:

[inline-code-attrs-start title = 'Endpoint OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Gli account nella regione EU usano `https://eu.fastcomments.com` come emittente, con gli stessi percorsi.

### Registrazione di un client

Un client ha bisogno di un `client_id` e di un `redirect_uri` registrato prima di poter avviare il flusso. Ci sono due modi per ottenerne uno:

- **Registrazione dinamica del client.** `POST /oauth/register` con un corpo JSON secondo RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). La risposta contiene il `client_id` e, per i client confidenziali, il `client_secret`. La registrazione è non autenticata e limitata per IP.
- **Documento di metadati del client ID.** Il client utilizza un URL `https` che controlla come suo `client_id`. FastComments recupera quell'URL e legge gli stessi campi di metadati. Non è necessaria alcuna chiamata di registrazione.

Le applicazioni partner elencate nella dashboard di FastComments, come Zapier, sono registrate direttamente da FastComments. Contatta il supporto se stai creando un'inserzione di marketplace e hai bisogno di un client di prima parte.

### Scope

[inline-code-attrs-start title = 'Scope'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Una richiesta che non specifica alcuno scope ottiene entrambi. L'utente vede gli scope richiesti nella pagina di consenso. Una richiesta per uno scope diverso da questi due fallisce con `invalid_scope`.

### Passo 1 - Richiesta di autorizzazione

Invia il browser dell'utente all'endpoint di autorizzazione. PKCE con il metodo `S256` è obbligatorio per ogni client.

[inline-code-attrs-start title = 'Richiesta di Autorizzazione'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parametri della Richiesta di Autorizzazione'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

L'utente accede a FastComments se necessario e vede una pagina di consenso che indica la tua applicazione, l'account a cui sarà collegata e gli scope richiesti. L'utente deve possedere il permesso **API Admin** su quell'account; chiunque altro vede un errore di permesso invece del modulo di consenso. L'approvazione reindirizza il browser al tuo `redirect_uri` con `code` e `state`. Il rifiuto reindirizza con `error=access_denied`.

Il codice di autorizzazione è valido per 10 minuti e può essere scambiato una sola volta. Un secondo scambio dello stesso codice revoca tutti i token prodotti dal primo scambio.

### Passo 2 - Richiesta di token

Scambia il codice per i token. Il corpo è codificato come form. I client confidenziali si autenticano con `client_secret_basic` (HTTP Basic) o `client_secret_post` (segreto nel corpo). I client pubblici inviano solo `client_id`.

[inline-code-attrs-start title = 'Esempio cURL della Richiesta di Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Corpo della Richiesta di Token (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struttura della Risposta del Token'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Gli errori seguono RFC 6749: un corpo JSON con `error` e `error_description`, HTTP 400 per `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` e `unsupported_grant_type`, HTTP 401 per `invalid_client`, HTTP 429 quando limitati per velocità.

### Passo 3 - Chiamare l'API

Invia il token di accesso come token bearer. Il tenant è implicito nel token, quindi `tenantId` è opzionale. Se fornito deve corrispondere al token altrimenti la richiesta fallisce.

[inline-code-attrs-start title = 'Esempio cURL del Token Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` restituisce il tenant, l'utente autorizzatore e gli scope concessi, il che lo rende la chiamata giusta per un test di connessione. Una richiesta con un token scaduto o revocato restituisce HTTP 401. Una richiesta il cui metodo richiede uno scope non posseduto dal token restituisce HTTP 403.

### Passo 4 - Aggiornamento

[inline-code-attrs-start title = 'Esempio cURL della Richiesta di Refresh'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Corpo della Richiesta di Token (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

La risposta ha la stessa struttura dello scambio del codice. I refresh token ruotano: ogni refresh restituisce un nuovo `refresh_token` e revoca quello vecchio dopo una finestra di grazia di 30 secondi per richieste concorrenti. Presentare un refresh token ruotato più di 30 secondi fa sì che venga trattato come replay e revoca l'intero grant. Le applicazioni partner registrate da FastComments sono esenti dalla rotazione e ricevono lo stesso refresh token con la scadenza estesa di altri 30 giorni.

Un refresh verifica anche che l'utente autorizzatore mantenga ancora il permesso API Admin sull'account. In caso contrario, il grant è revocato e la risposta è `invalid_grant`.

### Revoca

[inline-code-attrs-start title = 'Esempio cURL della Richiesta di Revoca'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Revocare un refresh token revoca tutti gli access token emessi dallo stesso grant. Revocare un access token revoca solo quel token. L'endpoint restituisce HTTP 200 con un oggetto JSON vuoto, indipendentemente dal fatto che il token sia stato trovato, secondo RFC 7009.

Gli utenti possono anche revocare una connessione da **App connesse** nella dashboard di FastComments. Tutti i token per quell'applicazione smettono di funzionare immediatamente.