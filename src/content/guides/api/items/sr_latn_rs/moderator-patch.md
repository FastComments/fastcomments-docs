[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint pruža mogućnost ažuriranja `Moderator` po `id`.

Ažuriranje `Moderator` ima sledeća ograničenja:

- Sledeće vrednosti ne smeju biti prosleđene prilikom ažuriranja `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Kada je naveden `userId`, taj korisnik mora postojati.
- Kada je naveden `userId`, on mora pripadati istom `tenantId` koji je naveden u query parametrima.
- Dva moderatora u istom tenantu ne mogu biti dodata sa istim `email`.
- Ne smete menjati `tenantId` povezan sa `Moderator`.

[inline-code-attrs-start title = 'Primer cURL zahteva za Moderator PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Moderator PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Moderator PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]