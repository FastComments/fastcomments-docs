---
[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Цей API-ендпоінт надає можливість розблокувати користувача, який написав певний коментар. Підтримується розблокування коментарів, написаних користувачами FastComments.com, SSO Users та Tenant Users.

Він підтримує параметр тіла запиту `commentIdsToCheck`, щоб перевірити, чи інші потенційно видимі коментарі на клієнті слід заблокувати/розблокувати після виконання цієї дії.

Примітки:

- Цей виклик завжди має виконуватися в контексті користувача. Користувач може бути FastComments.com User, SSO User або Tenant User.
- Параметр `userId` у запиті — це користувач, який *виконує розблокування*. Наприклад: `User A` хоче розблокувати `User B`. Передайте `userId=User A` та id коментаря, який написав `User B`.
- Повністю анонімні коментарі (без ідентифікатора користувача та без електронної пошти) не можуть бути заблоковані, і буде повернено помилку.

[inline-code-attrs-start title = 'Приклад cURL розблокування коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Приклад cURL розблокування анонімного коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для розблокування коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при розблокуванні коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Додається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Додається у разі помилки. **/
    reason?: string
    /** Якщо визначено commentIdsToCheck, записи в цій мапі зі значенням true все ще заблоковані. Якщо значення false, можливо, варто знову показати коментарі користувачеві, щоб йому не довелося оновлювати сторінку. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---