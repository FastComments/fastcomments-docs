FastComments jest serwerem autoryzacji OAuth 2.1. Aplikacja może uzyskać token powiązany z jednym kontem FastComments i używać go na każdym endpointzie w tym przewodniku zamiast klucza API. Tak właśnie aplikacja Zapier, serwer MCP i inne integracje firm trzecich się łączą.

Tokeny są wydawane w ramach przepływu kodu autoryzacji z PKCE. Nie ma przyznawania poświadczeń klienta ani przyznawania niejawnego.

### Discovery

Lokalizacje endpointów, obsługiwane przyznania i metody uwierzytelniania są publikowane pod standardowym adresem metadanych:

[inline-code-attrs-start title = 'Metadane serwera autoryzacji'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Endpointy, które opisuje:

[inline-code-attrs-start title = 'Punkty końcowe OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Konta w regionie UE używają `https://eu.fastcomments.com` jako wystawcy, z tymi samymi ścieżkami.

### Registering a client

Klient potrzebuje `client_id` i zarejestrowanego `redirect_uri`, zanim będzie mógł rozpocząć przepływ. Istnieją dwa sposoby, aby je uzyskać:

- **Dynamic Client Registration.** `POST /oauth/register` z ciałem JSON zgodnym z RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Odpowiedź zawiera `client_id` oraz, dla klientów poufnych, `client_secret`. Rejestracja jest nieuwierzytelniona i limitowana według IP.
- **Client ID Metadata Document.** Klient używa kontrolowanego przez siebie adresu URL `https` jako swojego `client_id`. FastComments pobiera ten URL i odczytuje z niego te same pola metadanych. Nie jest potrzebne wywołanie rejestracji.

Aplikacje partnerskie wymienione w panelu FastComments, takie jak Zapier, są rejestrowane bezpośrednio przez FastComments. Skontaktuj się z pomocą techniczną, jeśli tworzysz wpis w marketplace i potrzebujesz klienta pierwszej strony.

### Scopes

[inline-code-attrs-start title = 'Zakresy'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Żądanie, które nie podaje zakresu, otrzymuje oba. Użytkownik widzi żądane zakresy na stronie zgody. Żądanie zakresu innego niż te dwa kończy się błędem `invalid_scope`.

### Step 1 - Authorization request

Wyślij przeglądarkę użytkownika do endpointu autoryzacji. PKCE z metodą `S256` jest wymagane dla każdego klienta.

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
    /** Musi dokładnie pasować do jednego z zarejestrowanych URI przekierowania klienta. **/
    redirect_uri: string
    /** Space separated. Omit to request both scopes. **/
    /** Oddzielone spacjami. Pomiń, aby żądać obu zakresów. **/
    scope?: 'read' | 'write' | 'read write'
    /** Returned unchanged on the redirect. Use it to bind the callback to the session that started the flow. **/
    /** Zwracane niezmienione w przekierowaniu. Użyj go, aby powiązać wywołanie zwrotne z sesją, która rozpoczęła przepływ. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optional RFC 8707 resource indicator. If sent, the same value must be sent to the token endpoint. **/
    /** Opcjonalny wskaźnik zasobu RFC 8707. Jeśli jest wysyłany, ta sama wartość musi być wysłana do punktu końcowego tokena. **/
    resource?: string
}
[inline-code-end]

Użytkownik loguje się do FastComments w razie potrzeby i widzi stronę zgody, na której wymieniona jest Twoja aplikacja, konto, z którym zostanie połączona, oraz żądane zakresy. Użytkownik musi posiadać uprawnienie **API Admin** na tym koncie; każdy inny zobaczy błąd uprawnień zamiast formularza zgody. Zatwierdzenie przekierowuje przeglądarkę do Twojego `redirect_uri` z parametrami `code` i `state`. Odrzucenie przekierowuje z `error=access_denied`.

Kod autoryzacji jest ważny przez 10 minut i może być wymieniony jednorazowo. Druga wymiana tego samego kodu unieważnia każdy token, który został wyprodukowany przy pierwszej wymianie.

### Step 2 - Token request

Wymień kod na tokeny. Ciało jest kodowane jako formularz. Klienci poufni uwierzytelniają się przy pomocy `client_secret_basic` (HTTP Basic) lub `client_secret_post` (sekret w ciele). Klienci publiczni wysyłają tylko `client_id`.

[inline-code-attrs-start title = 'Przykład żądania tokena cURL'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
    /** Tylko dla klientów poufnych. Może być wysłane jako uwierzytelnienie HTTP Basic. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Must match the authorization request when sent. **/
    /** Musi pasować do żądania autoryzacji, gdy jest wysyłane. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tokena'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefixed fcat_. Valid for one hour. **/
    /** Z prefiksem fcat_. Ważny przez jedną godzinę. **/
    access_token: string
    token_type: 'bearer'
    /** Seconds until the access token expires. 3600. **/
    /** Sekundy do wygaśnięcia tokena dostępu. 3600. **/
    expires_in: number
    /** Prefixed fcrt_. Valid for 30 days from issue. **/
    /** Z prefiksem fcrt_. Ważny przez 30 dni od wydania. **/
    refresh_token: string
    /** Space separated scopes granted. **/
    /** Zakresy przyznane, oddzielone spacjami. **/
    scope: string
}
[inline-code-end]

Błędy są zgodne z RFC 6749: ciało JSON z `error` i `error_description`, HTTP 400 dla `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` i `unsupported_grant_type`, HTTP 401 dla `invalid_client`, HTTP 429 przy limitowaniu.

### Step 3 - Calling the API

Wyślij token dostępu jako token typu bearer. Tenant jest implikowany przez token, więc `tenantId` jest opcjonalny. Jeśli zostanie podany, musi pasować do tokena, w przeciwnym razie żądanie się nie powiedzie.

[inline-code-attrs-start title = 'Przykład tokena Bearer cURL'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` zwraca tenant, autoryzującego użytkownika oraz przyznane zakresy, co czyni to właściwym wywołaniem do testu połączenia. Żądanie z wygasłym lub unieważnionym tokenem zwraca HTTP 401. Żądanie, którego metoda wymaga zakresu, którego token nie posiada, zwraca HTTP 403.

### Step 4 - Refreshing

[inline-code-attrs-start title = 'Przykład żądania odświeżenia cURL'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
    /** Opcjonalny. Ogranicza do podzbioru pierwotnie przyznanych zakresów. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Odpowiedź ma taką samą strukturę jak wymiana kodu. Tokeny odświeżania rotują: każde odświeżenie zwraca nowy `refresh_token` i unieważnia stary po 30‑sekundowym oknie łaski dla równoczesnych żądań. Przedstawienie tokena odświeżania, który został obrócony ponad 30 sekund temu, jest traktowane jako powtórka i unieważnia cały przyznany dostęp. Aplikacje partnerskie zarejestrowane przez FastComments są zwolnione z rotacji i otrzymują ten sam token odświeżania z przedłużonym terminem ważności o kolejne 30 dni.

Odświeżenie ponownie sprawdza, czy autoryzujący użytkownik nadal posiada uprawnienie API Admin na koncie. Jeśli nie, przyznanie jest unieważniane, a odpowiedź to `invalid_grant`.

### Revocation

[inline-code-attrs-start title = 'Przykład żądania unieważnienia cURL'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Unieważnienie tokena odświeżania unieważnia każdy token dostępu wydany w ramach tego samego przyznania. Unieważnienie tokena dostępu unieważnia tylko ten token. Endpoint zwraca HTTP 200 z pustym obiektem JSON, niezależnie od tego, czy token został znaleziony, zgodnie z RFC 7009.

Użytkownicy mogą także unieważnić połączenie w **Connected Apps** w panelu FastComments. Każdy token dla tej aplikacji przestaje działać natychmiast.