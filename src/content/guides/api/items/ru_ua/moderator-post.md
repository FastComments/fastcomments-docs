[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Этот маршрут позволяет добавить одного `Moderator`.

При создании `Moderator` действуют следующие ограничения:

- Всегда необходимо указывать `name` и `email`. `userId` — необязательный.
- Следующие значения не могут быть указаны при создании `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Если указан `userId`, такой пользователь должен существовать.
- Если указан `userId`, он должен принадлежать тому же `tenantId`, который указан в параметрах запроса.
- Нельзя добавить двух модераторов в одном tenant с одинаковым `email`.

Мы можем создать `Moderator` для пользователя, о котором известен только email:

[inline-code-attrs-start title = 'Пример cURL: создание Moderator по электронной почте'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Или мы можем создать `Moderator` для пользователя, который принадлежит нашему tenant, чтобы отслеживать его статистику модерации:

[inline-code-attrs-start title = 'Пример cURL: создание Moderator для пользователя tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запроса на создание Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при создании Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Указывается в случае ошибки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Указывается в случае ошибки. **/
    reason?: string
    moderator?: Moderator; // В случае успеха возвращаем полностью созданного модератора.
}
[inline-code-end]

---