[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ця кінцева точка API дозволяє оновити `Moderator` за `id`.

Оновлення `Moderator` має такі обмеження:

- Під час оновлення `Moderator` не можна надавати такі значення:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Якщо вказано `userId`, цей користувач має існувати.
- Якщо вказано `userId`, він має належати до того ж `tenantId`, що вказано в параметрах запиту.
- Двоє модераторів у тому ж tenant не можуть мати однаковий `email`.
- Ви не можете змінювати `tenantId`, пов'язаний з `Moderator`.

[inline-code-attrs-start title = 'Приклад cURL-запиту PATCH для Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту PATCH для Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді PATCH для Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Додається у випадку невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Додається у випадку невдачі. **/
    reason?: string
}
[inline-code-end]