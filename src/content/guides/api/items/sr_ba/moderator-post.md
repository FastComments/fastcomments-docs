[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава додавање једног `Moderator`.

Креирање `Moderator`-а има сљедећа ограничења:

- Морају увијек бити наведени `name` и `email`. `userId` је опционо.
- Сљедеће вриједности не смију бити наведене при креирању `Moderator`-а:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Када је `userId` назначен, тај корисник мора постојати.
- Када је `userId` назначен, он мора припадати истом `tenantId`-у наведеном у параметрима упита.
- Два модератора у истом тенанту не могу бити додата са истим `email`-ом.

Можемо креирати `Moderator`-а за корисника чији познајемо само `email`:

[inline-code-attrs-start title = 'Пример cURL захтева за креирање модератора преко е-поште'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Или можемо креирати `Moderator`-а за корисника који припада нашем тенанту, како бисмо пратили њихове статистике модерирања:

[inline-code-attrs-start title = 'Пример cURL захтева за креирање модератора преко корисника тенанта'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура захтева за креирање модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора при креирању модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    moderator?: Moderator; // При успјеху враћамо комплетног креираног модератора.
}
[inline-code-end]