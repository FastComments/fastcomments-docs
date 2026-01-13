[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Этот конечный пункт API предоставляет возможность обновить `Moderator` по `id`.

Обновление `Moderator` имеет следующие ограничения:

- Следующие значения не могут быть переданы при обновлении `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Когда указан `userId`, такой пользователь должен существовать.
- Когда указан `userId`, он должен принадлежать тому же `tenantId`, который указан в параметрах запроса.
- Двух модераторов в одном и том же тенанте нельзя добавить с одинаковым `email`.
- Вы не можете изменить `tenantId`, связанный с `Moderator`.

[inline-code-attrs-start title = 'Пример cURL-запроса PATCH для Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса PATCH для Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа PATCH для Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Включается при неудаче. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Включается при неудаче. **/
    reason?: string
}
[inline-code-end]