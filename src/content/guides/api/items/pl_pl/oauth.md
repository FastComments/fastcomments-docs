FastComments jest serwerem autoryzacji OAuth 2.1. Aplikacja może uzyskać token powiązany z jednym kontem FastComments i używać go na każdym punkcie końcowym w tym przewodniku zamiast klucza API. W ten sposób łączą się aplikacja Zapier, serwer MCP i inne integracje firm trzecich.

Tokeny są wydawane w przepływie kodu autoryzacji z PKCE. Nie ma grantu client credentials ani implicit.

### Discovery

Lokalizacje punktów końcowych, obsługiwane granty i metody uwierzytelniania są publikowane pod standardowym adresem URL metadanych:

[inline-code-attrs-start title = 'Metadane serwera autoryzacji'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Punkty końcowe, które opisuje:

[inline-code-attrs-start title = 'Punkty końcowe OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Konta w regionie UE używają `https://eu.fastcomments.com` jako issuer, z tymi samymi ścieżkami.

### Rejestracja klienta

Klient potrzebuje `client_id` i zarejestrowanego `redirect_uri`, zanim rozpocznie przepływ. Są dwa sposoby, aby je uzyskać:

- **Dynamiczna rejestracja klienta.** `POST /oauth/register` z treścią JSON zgodnie z RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Odpowiedź zawiera `client_id`, a dla klientów poufnych także `client_secret`. Rejestracja jest nieuwierzytelniona i ograniczona limitem na IP.
- **Client ID Metadata Document.** Klient używa kontrolowanego przez siebie adresu URL `https` jako swojego `client_id`. FastComments pobiera ten adres i odczytuje z niego te same pola metadanych. Wywołanie rejestracji nie jest potrzebne.

Aplikacje partnerskie wymienione w panelu FastComments, takie jak Zapier, są rejestrowane bezpośrednio przez FastComments. Skontaktuj się z pomocą techniczną, jeśli budujesz wpis w marketplace i potrzebujesz klienta pierwszej strony.

### Zakresy (scopes)

[inline-code-attrs-start title = 'Zakresy'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   żądania GET i HEAD
write  każda inna metoda
[inline-code-end]

Żądanie bez określonego zakresu otrzymuje oba. Użytkownik widzi żądane zakresy na stronie zgody. Żądanie zakresu innego niż te dwa kończy się błędem `invalid_scope`.

### Krok 1 - Żądanie autoryzacji

Przekieruj przeglądarkę użytkownika do punktu końcowego autoryzacji. PKCE z metodą `S256` jest wymagane dla każdego klienta.

[inline-code-attrs-start title = 'Żądanie autoryzacji'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Parametry żądania autoryzacji'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Użytkownik loguje się do FastComments, jeśli to konieczne, i widzi stronę zgody z nazwą Twojej aplikacji, kontem, z którym zostanie połączona, oraz żądanymi zakresami. Użytkownik musi posiadać uprawnienie **API Admin** na tym koncie; każdy inny zobaczy błąd uprawnień zamiast formularza zgody. Zatwierdzenie przekierowuje przeglądarkę do Twojego `redirect_uri` z `code` i `state`. Odmowa przekierowuje z `error=access_denied`.

Kod autoryzacji jest ważny 10 minut i można go wymienić tylko raz. Druga wymiana tego samego kodu odwołuje wszystkie tokeny wyprodukowane przez pierwszą wymianę.

### Krok 2 - Żądanie tokena

Wymień kod na tokeny. Treść jest zakodowana jako formularz. Klienci poufni uwierzytelniają się przez `client_secret_basic` (HTTP Basic) lub `client_secret_post` (sekret w treści). Klienci publiczni wysyłają tylko `client_id`.

[inline-code-attrs-start title = 'Przykład cURL żądania tokena'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Treść żądania tokena (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odpowiedzi tokena'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Błędy zgodnie z RFC 6749: treść JSON z `error` i `error_description`, HTTP 400 dla `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` i `unsupported_grant_type`, HTTP 401 dla `invalid_client`, HTTP 429 przy ograniczeniu limitu.

### Krok 3 - Wywoływanie API

Wysyłaj token dostępu jako bearer token. Najemca (tenant) jest implikowany przez token, więc `tenantId` jest opcjonalne. Jeśli podane, musi zgadzać się z tokenem, inaczej żądanie kończy się błędem.

[inline-code-attrs-start title = 'Przykład cURL z bearer tokenem'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` zwraca najemcę, autoryzującego użytkownika i przyznane zakresy, co czyni go właściwym wywołaniem do testu połączenia. Żądanie z wygasłym lub odwołanym tokenem otrzymuje HTTP 401. Żądanie, którego metoda wymaga zakresu, którego token nie posiada, otrzymuje HTTP 403.

### Krok 4 - Odświeżanie

[inline-code-attrs-start title = 'Przykład cURL żądania odświeżenia'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Treść żądania tokena (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Odpowiedź ma ten sam kształt co wymiana kodu. Refresh tokeny rotują: każde odświeżenie zwraca nowy `refresh_token` i odwołuje stary po 30-sekundowym oknie łaski dla żądań współbieżnych. Przedstawienie refresh tokena rotowanego ponad 30 sekund temu jest traktowane jako replay i odwołuje cały grant. Aplikacje partnerskie zarejestrowane przez FastComments są zwolnione z rotacji i otrzymują z powrotem ten sam refresh token z przedłużonym o kolejne 30 dni terminem ważności.

Odświeżenie ponownie sprawdza, czy autoryzujący użytkownik nadal posiada API Admin na koncie. Jeśli nie, grant jest odwoływany, a odpowiedź to `invalid_grant`.

### Odwoływanie

[inline-code-attrs-start title = 'Przykład cURL żądania odwołania'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Odwołanie refresh tokena odwołuje wszystkie tokeny dostępu wydane z tego samego grantu. Odwołanie tokena dostępu odwołuje tylko ten token. Punkt końcowy zwraca HTTP 200 z pustym obiektem JSON niezależnie od tego, czy token został znaleziony, zgodnie z RFC 7009.

Użytkownicy mogą również odwołać połączenie z poziomu **Connected Apps** w panelu FastComments. Wszystkie tokeny tej aplikacji przestają działać natychmiast.