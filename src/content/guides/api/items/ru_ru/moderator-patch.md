[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Этот эндпоинт API позволяет обновить `Moderator` по `id`.

Обновление `Moderator` имеет следующие ограничения:

- Следующие значения не могут быть указаны при обновлении `Moderator`:
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
- Нельзя добавить двух модераторов в одном и том же `tenant` с одинаковым `email`.
- Нельзя изменить `tenantId`, связанный с `Moderator`.

[inline-code-attrs-start title = 'Пример cURL-запроса для Moderator PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Moderator PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Moderator PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Указывается при ошибке. **/
    reason?: string
}
[inline-code-end]

---