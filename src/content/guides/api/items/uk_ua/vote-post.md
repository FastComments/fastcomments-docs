[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Цей маршрут надає можливість додати один авторизований `Vote`. Голоси можуть бути `up` (+1) або `down` (-1).

[inline-code-attrs-start title = 'Приклад cURL для створення голосу'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Приклад cURL для створення анонімного голосу'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту створення голосу'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді на створення голосу'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Створення анонімних голосів

Анонімні голоси можна створити, вказавши `anonUserId` у параметрах запиту замість `userId`.

Цей ідентифікатор не має відповідати жодному об'єкту користувача (звідси й «анонімний»). Це просто ідентифікатор
сесії, тож ви можете знову отримати голоси в тій самій сесії, щоб перевірити, чи був
проголосований коментар.

Якщо у вас немає такого поняття, як «анонімні сесії», як у FastComments — ви можете просто
встановити випадковий ідентифікатор, наприклад UUID (хоча ми віддаємо перевагу коротшим ідентифікаторам для економії місця).

### Інші примітки

- Цей API дотримується налаштувань на рівні тенанта. Наприклад, якщо ви вимкнули голосування для певної сторінки і спробуєте створити голос через API, це завершиться помилкою з кодом `voting-disabled`.
- Цей API є «живим» за замовчуванням.
- Цей API оновлює поле `votes` відповідного `Comment`.