[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Този маршрут предоставя възможност за добавяне на единичен `Moderator`.

Създаването на `Moderator` има следните ограничения:

- Винаги трябва да се предоставят `name` и `email`. `userId` е незадължителен.
- Следните стойности не могат да бъдат предоставени при създаване на `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Когато е посочен `userId`, този потребител трябва да съществува.
- Когато е посочен `userId`, той трябва да принадлежи на същия `tenantId`, посочен в параметрите на заявката.
- Двама модератори в един и същ tenant не могат да бъдат добавени с един и същ `email`.

Можем да създадем `Moderator` за потребител, за който знаем само имейла:

[inline-code-attrs-start title = 'Пример за създаване на Moderator чрез имейл с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Или можем да създадем `Moderator` за потребител, който принадлежи на нашия tenant, за да проследяваме неговите статистики за модерация:

[inline-code-attrs-start title = 'Пример за създаване на Moderator чрез Tenant потребител с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на заявката за създаване на Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за създаване на Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    moderator?: Moderator; // We return the complete created moderator on success.
}
[inline-code-end]
