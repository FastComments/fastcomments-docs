[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ова крајња тачка API-ја омогућава ажурирање `Moderator` по `id`.

Ажурирање `Moderator` има следећа ограничења:

- Следеће вредности не смеју бити прослеђене приликом ажурирања `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Када је наведено `userId`, тај корисник мора постојати.
- Када је наведено `userId`, они морају припадати истом `tenantId` наведеном у параметрима упита.
- Два модератора у истом tenant-у не могу бити додата са истим `email`.
- Не можете променити `tenantId` повезан са `Moderator`-ом.

[inline-code-attrs-start title = 'Пример cURL PATCH захтева за Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH захтева за Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора PATCH захтева за Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---