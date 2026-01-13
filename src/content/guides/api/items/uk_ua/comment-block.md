---
[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Цей API-ендпоінт надає можливість заблокувати користувача, який написав певний коментар. Підтримується блокування коментарів, написаних користувачами FastComments.com, SSO-користувачами та користувачами орендаря.

Підтримується параметр тіла `commentIdsToCheck`, щоб перевірити, чи інші потенційно видимі коментарі на клієнті повинні бути заблоковані/розблоковані після виконання цієї дії.

Примітки:

- Цей виклик завжди має виконуватися в контексті користувача. Користувач може бути користувачем FastComments.com, SSO-користувачем або користувачем орендаря.
- `userId` у запиті — це користувач, який *виконує блокування*. Наприклад: `User A` хоче заблокувати `User B`. Передайте `userId=User A` та id коментаря, який написав `User B`.
- Повністю анонімні коментарі (немає id користувача, немає електронної пошти) не можна заблокувати — буде повернено помилку.

[inline-code-attrs-start title = 'Приклад cURL-запиту для блокування коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонімного блокування ми маємо вказати `anonUserId`. Це може бути ідентифікатор, який представляє анонімну сесію, або випадковий UUID.
Це дозволяє нам підтримувати блокування коментарів навіть якщо користувач не увійшов, отримуючи коментарі з тим самим `anonUserId`.

[inline-code-attrs-start title = 'Приклад cURL-запиту для анонімного блокування коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту блокування коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при блокуванні коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Включається у разі помилки. **/
    reason?: string
    /** Якщо commentIdsToCheck визначено, записи в цій мапі зі значенням true також будуть заблоковані. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---