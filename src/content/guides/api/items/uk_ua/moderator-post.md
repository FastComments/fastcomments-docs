---
[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Цей маршрут надає можливість додати одного `Moderator`.

Створення `Moderator` має такі обмеження:

- Потрібно завжди вказувати `name` та `email`. `userId` є необов'язковим.
- При створенні `Moderator` не можна вказувати такі значення:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Якщо вказано `userId`, такий користувач повинен існувати.
- Якщо вказано `userId`, він має належати тому самому `tenantId`, що вказаний у параметрах запиту.
- Не можна додати двох модераторів у тому самому тенанті з однаковим `email`.

Ми можемо створити `Moderator` для користувача, якого ми знаємо тільки за електронною поштою:

[inline-code-attrs-start title = 'Приклад cURL: створення Модератора за електронною поштою'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Або ми можемо створити `Moderator` для користувача, який належить нашому тенанту, щоб відстежувати його статистику модерації:

[inline-code-attrs-start title = 'Приклад cURL: створення Модератора для користувача тенанта'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для створення Модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при створенні Модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Включено у випадку помилки. **/
    reason?: string
    moderator?: Moderator; // Ми повертаємо повністю створеного модератора при успіху.
}
[inline-code-end]

---