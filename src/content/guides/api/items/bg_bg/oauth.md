FastComments е сървър за упълномощаване OAuth 2.1. Приложение може да получи токен, който е свързан с един FastComments акаунт и да го използва за всеки крайна точка в това ръководство вместо API ключ. По този начин Zapier приложението, MCP сървърът и други интеграции от трети страни се свързват.

Токените се издават чрез потока за код за упълномощаване с PKCE. Няма клиентски идентификационни данни или имплицитно предоставяне.

### Откриване

Местоположенията на крайните точки, поддържаните предоставяния и методи за упълномощаване се публикуват на стандартния URL за метаданни:

[inline-code-attrs-start title = 'Метаданни на сървъра за упълномощаване'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Крайните точки, които описва:

[inline-code-attrs-start title = 'OAuth крайни точки'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Акаунтите в регион EU използват `https://eu.fastcomments.com` като издател, със същите пътища.

### Регистриране на клиент

Клиентът се нуждае от `client_id` и регистриран `redirect_uri`, преди да може да започне потока. Има два начина да се получи такъв:

- **Динамична регистрация на клиент.** `POST /oauth/register` с JSON тяло според RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Отговорът съдържа `client_id` и, за поверителни клиенти, `client_secret`. Регистрацията е неаутентицирана и има ограничение за честота по IP.
- **Документ с метаданни за клиентски идентификатор.** Клиентът използва `https` URL, който контролира, като свой `client_id`. FastComments извлича този URL и чете същите полета за метаданни от него. Не е необходима заявка за регистрация.

Партньорски приложения, изброени в таблото на FastComments, като Zapier, се регистрират директно от FastComments. Свържете се с поддръжката, ако създавате листинг в пазар и ви е нужен клиент от първа страна.

### Обхвати

[inline-code-attrs-start title = 'Обхвати'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Заявка, която не изисква обхват, получава и двете. Потребителят вижда заявените обхвати на страницата за съгласие. Заявка за обхват, различен от тези два, се отказва с `invalid_scope`.

### Стъпка 1 – Заявка за упълномощаване

Пренасочете браузъра на потребителя към крайната точка за упълномощаване. PKCE с метод `S256` е задължителен за всеки клиент.

[inline-code-attrs-start title = 'Заявка за упълномощаване'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Параметри на заявката за упълномощаване'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Трябва точно да съвпада с един от регистрираните за клиента URI за пренасочване. **/
    redirect_uri: string
    /** Разделени с интервал. Пропуснете, за да поискате и двата обхвата. **/
    scope?: 'read' | 'write' | 'read write'
    /** Връща се непроменено в пренасочването. Използвайте го, за да свържете обратния повик към сесията, която стартира потока. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Незадължителен индикатор за ресурс според RFC 8707. Ако се изпрати, същата стойност трябва да се изпрати към крайната точка за токен. **/
    resource?: string
}
[inline-code-end]

Потребителят се вписва в FastComments, ако е необходимо, и вижда страница за съгласие, която назовава вашето приложение, акаунта, към който ще се свърже, и заявените обхвати. Потребителят трябва да притежава правото **API Admin** за този акаунт; всеки друг ще види грешка за разрешения вместо формуляра за съгласие. Одобряването пренасочва браузъра към вашия `redirect_uri` с `code` и `state`. Отказът пренасочва с `error=access_denied`.

Кодът за упълномощаване е валиден 10 минути и може да се обменя веднъж. Вторият обмен на същия код отмяна всеки токен, създаден от първия обмен.

### Стъпка 2 – Заявка за токен

Разменете кода за токени. Тялото е форм-енкодирано. Поверителните клиенти се удостоверяват с `client_secret_basic` (HTTP Basic) или `client_secret_post` (секрет в тялото). Публичните клиенти изпращат само `client_id`.

[inline-code-attrs-start title = 'Пример за cURL заявка за токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Тяло на заявка за токен (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Само за поверителни клиенти. Може да се изпрати като HTTP Basic удостоверяване вместо това. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Трябва да съвпада с заявката за упълномощаване, когато се изпрати. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за токен'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Префикс fcat_. Валиден за един час. **/
    access_token: string
    token_type: 'bearer'
    /** Секунди до изтичане на токена за достъп. 3600. **/
    expires_in: number
    /** Префикс fcrt_. Валиден 30 дни от издаването. **/
    refresh_token: string
    /** Обхвати, разделени с интервал, предоставени. **/
    scope: string
}
[inline-code-end]

Грешките следват RFC 6749: JSON тяло с `error` и `error_description`, HTTP 400 за `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` и `unsupported_grant_type`, HTTP 401 за `invalid_client`, HTTP 429 при ограничение на честотата.

### Стъпка 3 – Извикване на API

Изпратете токена за достъп като носител токен. Наемателят се подразбира от токена, така че `tenantId` е незадължителен. Когато се предостави, трябва да съвпада с токена, иначе заявката се отказва.

[inline-code-attrs-start title = 'Пример за cURL заявка с носител токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` връща наемателя, упълномощаващия потребител и предоставените обхвати, което я прави подходяща за тест на връзка. Заявка с изтекъл или оттеглен токен получава HTTP 401. Заявка, чий метод изисква обхват, който токенът не притежава, получава HTTP 403.

### Стъпка 4 – Обновяване

[inline-code-attrs-start title = 'Пример за cURL заявка за обновяване'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Тяло на заявка за токен (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Незадължително. Ограничаване до подмножество от първоначално предоставените обхвати. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Отговорът има същата структура като обмена на код. Токените за обновяване се въртят: всяко обновяване връща нов `refresh_token` и оттегля стария след 30‑секунден прозорец за едновременни заявки. Представянето на токен за обновяване, който е въртян преди повече от 30 секунди, се счита за повторно използване и оттегля цялото предоставяне. Партньорски приложения, регистрирани от FastComments, са изключени от въртенето и получават същия токен за обновяване с удължена валидност с още 30 дни.

Обновяването също проверява дали упълномощаващият потребител все още притежава API Admin за акаунта. Ако не, предоставянето се оттегля и отговорът е `invalid_grant`.

### Отмяна

[inline-code-attrs-start title = 'Пример за cURL заявка за отмяна'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Отмяната на токен за обновяване отмяна всички токени за достъп, издадени от същото предоставяне. Отмяната на токен за достъп отмяна само този токен. Крайната точка връща HTTP 200 с празен JSON обект, независимо дали токенът е намерен, според RFC 7009.

Потребителите също могат да отмятат връзка от **Свързани приложения** в таблото на FastComments. Всеки токен за това приложение спира да работи незабавно.