FastComments є сервером авторизації OAuth 2.1. Додаток може отримати токен, який прив’язаний до одного облікового запису FastComments, і використовувати його на кожному кінцевому пункті в цьому посібнику замість API‑ключа. Так підключаються додаток Zapier, сервер MCP та інші сторонні інтеграції.

Токени видаються через потік коду авторизації з PKCE. Не передбачено клієнтських облікових даних або неявного гранту.

### Discovery

Розташування кінцевих точок, підтримувані гранти та методи автентифікації публікуються за стандартною URL‑адресою метаданих:

[inline-code-attrs-start title = 'Метадані сервера авторизації'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Кінцеві точки, які він описує:

[inline-code-attrs-start title = 'Кінцеві точки OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Облікові записи в регіоні ЄС використовують `https://eu.fastcomments.com` як видавця, з тими ж шляхами.

### Реєстрація клієнта

Клієнту потрібен `client_id` та зареєстрований `redirect_uri`, перш ніж він зможе розпочати процес. Існує два способи отримати їх:

- **Динамічна реєстрація клієнта.** `POST /oauth/register` з JSON‑тілом згідно RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Відповідь містить `client_id` і, для конфіденційних клієнтів, `client_secret`. Реєстрація не потребує автентифікації та обмежена за швидкістю per IP.
- **Документ метаданих Client ID.** Клієнт використовує `https` URL, яким він керує, як свій `client_id`. FastComments отримує цей URL і читає з нього ті ж поля метаданих. Виклик реєстрації не потрібен.

Партнерські додатки, зазначені в панелі FastComments, такі як Zapier, реєструються безпосередньо FastComments. Зверніться до підтримки, якщо ви створюєте запис у маркетплейсі і потрібен клієнт першої сторони.

### Скоупи

[inline-code-attrs-start title = 'Скоупи'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Запит, який не вказує жодного скоупу, отримує обидва. Користувач бачить запитані скоупи на сторінці згоди. Запит на скоуп, відмінний від цих двох, завершується помилкою `invalid_scope`.

### Крок 1 – Запит авторизації

Надішліть браузер користувача до кінцевої точки авторизації. PKCE з методом `S256` є обов’язковим для кожного клієнта.

[inline-code-attrs-start title = 'Запит авторизації'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Параметри запиту авторизації'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Користувач входить у FastComments за потреби і бачить сторінку згоди, на якій вказано ваш додаток, обліковий запис, до якого він буде підключений, та запитані скоупи. Користувач повинен мати дозвіл **API Admin** на цьому обліковому записі; інші користувачі бачать помилку дозволу замість форми згоди. Підтвердження перенаправляє браузер на ваш `redirect_uri` з параметрами `code` та `state`. Відмова перенаправляє з `error=access_denied`.

Код авторизації дійсний протягом 10 хвилин і може бути обміняний один раз. Другий обмін тим же кодом відкликає всі токени, створені під час першого обміну.

### Крок 2 – Запит токену

Обміняйте код на токени. Тіло запиту кодується у форматі form. Конфіденційні клієнти автентифікуються за допомогою `client_secret_basic` (HTTP Basic) або `client_secret_post` (секрет у тілі). Публічні клієнти надсилають лише `client_id`.

[inline-code-attrs-start title = 'Приклад cURL запиту токену'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Тіло запиту токену (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура відповіді токену'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Помилки відповідають RFC 6749: JSON‑тіло з `error` та `error_description`, HTTP 400 для `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` та `unsupported_grant_type`, HTTP 401 для `invalid_client`, HTTP 429 при обмеженні швидкості.

### Крок 3 – Виклик API

Надішліть токен доступу як токен типу bearer. Орендар (tenant) передбачений токеном, тому `tenantId` є необов’язковим. Якщо вказано, він має збігатися з токеном, інакше запит завершиться помилкою.

[inline-code-attrs-start title = 'Приклад cURL запиту Bearer токену'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` повертає орендаря, користувача, що авторизувався, та надані скоупи, що робить його правильним запитом для тесту підключення. Запит з простроченим або відкликаним токеном отримує HTTP 401. Запит, метод якого потребує скоупу, якого токен не має, отримує HTTP 403.

### Крок 4 – Оновлення

[inline-code-attrs-start title = 'Приклад cURL запиту оновлення'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Тіло запиту токену (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Відповідь має ту ж структуру, що й обмін кодом. Токени оновлення обертаються: кожне оновлення повертає новий `refresh_token` і відкликає старий після 30‑секундного вікна для одночасних запитів. Надання токену оновлення, який був обернений більше ніж 30 секунд тому, розглядається як повтор і відкликає весь грант. Партнерські додатки, зареєстровані FastComments, звільнені від обертання і отримують той самий токен оновлення з продовженим терміном дії ще на 30 днів.

Оновлення також перевіряє, чи користувач, що авторизував, все ще має дозвіл API Admin на обліковому записі. Якщо ні, грант відкликається, і відповідь містить `invalid_grant`.

### Відкликання

[inline-code-attrs-start title = 'Приклад cURL запиту відкликання'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Відкликання токену оновлення відкликає всі токени доступу, видані в рамках того ж гранту. Відкликання токену доступу відкликає лише цей токен. Кінцева точка повертає HTTP 200 з порожнім JSON‑об’єктом, незалежно від того, чи був токен знайдений, згідно RFC 7009.

Користувачі також можуть відкликати підключення у розділі **Connected Apps** в панелі FastComments. Усі токени для цього додатку перестають працювати одразу.