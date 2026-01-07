[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за актуализиране на `Moderator` по `id`.

Актуализирането на `Moderator` има следните ограничения:

- Следните стойности не могат да бъдат предоставени при актуализиране на `Moderator`:
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
- Не можете да променяте `tenantId`, свързан с `Moderator`.

[inline-code-attrs-start title = 'Пример за PATCH на Moderator с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за PATCH на Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за PATCH на Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
