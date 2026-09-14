FastComments ist ein OAuth 2.1‑Autorisierungsserver. Eine Anwendung kann ein Token erhalten, das an ein FastComments‑Konto gebunden ist, und es an jedem Endpunkt in diesem Leitfaden anstelle eines API‑Schlüssels verwenden. So verbinden sich die Zapier‑App, der MCP‑Server und andere Drittanbieter‑Integrationen.

Tokens werden über den Autorisierungscode‑Flow mit PKCE ausgestellt. Es gibt keinen Client‑Credentials‑ oder impliziten Grant.

### Entdeckung

Endpunktadressen, unterstützte Grants und Auth‑Methoden werden unter der standardmäßigen Metadaten‑URL veröffentlicht:

[inline-code-attrs-start title = 'Metadaten des Autorisierungsservers'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Die Endpunkte, die es beschreibt:

[inline-code-attrs-start title = 'OAuth-Endpunkte'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Konten in der EU‑Region verwenden `https://eu.fastcomments.com` als Aussteller, mit denselben Pfaden.

### Registrierung eines Clients

Ein Client benötigt eine `client_id` und eine registrierte `redirect_uri`, bevor er den Flow starten kann. Es gibt zwei Möglichkeiten, eine zu erhalten:

- **Dynamische Client‑Registrierung.** `POST /oauth/register` mit einem JSON‑Body gemäß RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Die Antwort enthält die `client_id` und, für vertrauliche Clients, das `client_secret`. Die Registrierung ist nicht authentifiziert und pro IP rate‑limitiert.
- **Client‑ID‑Metadaten‑Dokument.** Der Client verwendet eine von ihm kontrollierte `https`‑URL als seine `client_id`. FastComments ruft diese URL ab und liest dieselben Metadatenfelder daraus. Ein Registrierungsaufruf ist nicht erforderlich.

Partner‑Anwendungen, die im FastComments‑Dashboard aufgeführt sind, wie Zapier, werden von FastComments direkt registriert. Kontaktieren Sie den Support, wenn Sie ein Marketplace‑Listing erstellen und einen First‑Party‑Client benötigen.

### Bereiche

[inline-code-attrs-start title = 'Bereiche'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Ein Antrag, der keinen Bereich anfordert, erhält beide. Der Benutzer sieht die angeforderten Bereiche auf der Zustimmungsseite. Ein Antrag auf einen anderen Bereich als diese beiden schlägt mit `invalid_scope` fehl.

### Schritt 1 – Autorisierungsanfrage

Senden Sie den Browser des Benutzers zum Autorisierungs‑Endpunkt. PKCE mit der Methode `S256` ist für jeden Client erforderlich.

[inline-code-attrs-start title = 'Autorisierungsanfrage'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parameter der Autorisierungsanfrage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Muss exakt einer der vom Client registrierten Redirect‑URIs entsprechen. **/
    redirect_uri: string
    /** Durch Leerzeichen getrennt. Weglassen, um beide Bereiche anzufordern. **/
    scope?: 'read' | 'write' | 'read write'
    /** Unverändert im Redirect zurückgegeben. Verwenden Sie es, um den Callback an die Sitzung zu binden, die den Flow gestartet hat. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optionaler RFC‑8707‑Ressourcenindikator. Wenn gesendet, muss derselbe Wert an den Token‑Endpunkt gesendet werden. **/
    resource?: string
}
[inline-code-end]

Der Benutzer meldet sich bei FastComments an, falls nötig, und sieht eine Zustimmungsseite, die Ihre Anwendung, das Konto, mit dem sie verbunden wird, und die angeforderten Bereiche nennt. Der Benutzer muss die **API‑Admin**‑Berechtigung für dieses Konto besitzen; alle anderen sehen einen Berechtigungsfehler anstelle des Zustimmungsformulars. Durch die Zustimmung wird der Browser zu Ihrer `redirect_uri` mit `code` und `state` weitergeleitet. Eine Ablehnung leitet mit `error=access_denied` weiter.

Der Autorisierungscode ist 10 Minuten gültig und kann einmal ausgetauscht werden. Ein zweiter Austausch desselben Codes widerruft jedes Token, das der erste Austausch erzeugt hat.

### Schritt 2 – Token‑Anfrage

Tauschen Sie den Code gegen Tokens aus. Der Body ist formularkodiert. Vertrauliche Clients authentifizieren sich mit `client_secret_basic` (HTTP Basic) oder `client_secret_post` (Geheimnis im Body). Öffentliche Clients senden nur `client_id`.

[inline-code-attrs-start title = 'Token-Anfrage cURL-Beispiel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Token-Anfragekörper (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Nur für vertrauliche Clients. Kann stattdessen als HTTP‑Basic‑Auth gesendet werden. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Muss mit der Autorisierungsanfrage übereinstimmen, wenn gesendet. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Token-Antwort'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Präfix fcat_. Gültig für eine Stunde. **/
    access_token: string
    token_type: 'bearer'
    /** Sekunden bis das Zugriffstoken abläuft. 3600. **/
    expires_in: number
    /** Präfix fcrt_. Gültig für 30 Tage ab Ausstellung. **/
    refresh_token: string
    /** Durch Leerzeichen getrennte gewährte Bereiche. **/
    scope: string
}
[inline-code-end]

Fehler folgen RFC 6749: ein JSON‑Body mit `error` und `error_description`, HTTP 400 für `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` und `unsupported_grant_type`, HTTP 401 für `invalid_client`, HTTP 429 bei Rate‑Limiting.

### Schritt 3 – Aufruf der API

Senden Sie das Zugriffstoken als Bearer‑Token. Der Mandant wird durch das Token impliziert, daher ist `tenantId` optional. Wenn angegeben, muss es dem Token entsprechen, sonst schlägt die Anfrage fehl.

[inline-code-attrs-start title = 'Bearer-Token cURL-Beispiel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` gibt den Mandanten, den autorisierenden Benutzer und die gewährten Bereiche zurück, was es zum richtigen Aufruf für einen Verbindungstest macht. Eine Anfrage mit einem abgelaufenen oder widerrufenen Token erhält HTTP 401. Eine Anfrage, deren Methode einen Bereich erfordert, den das Token nicht besitzt, erhält HTTP 403.

### Schritt 4 – Aktualisierung

[inline-code-attrs-start title = 'Refresh-Anfrage cURL-Beispiel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Token-Anfragekörper (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Optional. Beschränkt auf eine Teilmenge der ursprünglich gewährten Bereiche. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Die Antwort hat dieselbe Struktur wie der Code‑Austausch. Refresh‑Tokens rotieren: Jeder Refresh gibt ein neues `refresh_token` zurück und widerruft das alte nach einem 30‑Sekunden‑Grace‑Period für gleichzeitige Anfragen. Die Vorlage eines Refresh‑Tokens, das vor mehr als 30 Sekunden rotiert wurde, wird als Replay behandelt und widerruft das gesamte Grant. Partner‑Anwendungen, die von FastComments registriert wurden, sind von der Rotation ausgenommen und erhalten dasselbe Refresh‑Token, dessen Ablauf um weitere 30 Tage verlängert wird.

Ein Refresh prüft zudem erneut, ob der autorisierende Benutzer weiterhin API‑Admin für das Konto ist. Wenn nicht, wird das Grant widerrufen und die Antwort ist `invalid_grant`.

### Widerruf

[inline-code-attrs-start title = 'Widerruf-Anfrage cURL-Beispiel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Das Widerrufen eines Refresh‑Tokens widerruft jedes Zugriffstoken, das aus demselben Grant ausgestellt wurde. Das Widerrufen eines Zugriffstokens widerruft nur dieses Token. Der Endpunkt gibt HTTP 200 mit einem leeren JSON‑Objekt zurück, unabhängig davon, ob das Token gefunden wurde, gemäß RFC 7009.

Benutzer können auch eine Verbindung über **Connected Apps** im FastComments‑Dashboard widerrufen. Jeder Token für diese Anwendung hört sofort auf zu funktionieren.