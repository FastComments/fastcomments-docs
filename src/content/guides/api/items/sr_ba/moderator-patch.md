[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava ažuriranje `Moderator` po `id`.

Ažuriranje `Moderator` ima sljedeća ograničenja:

- Sljedeće vrijednosti se ne smiju dostaviti prilikom ažuriranja `Moderator`:
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
- Dva moderatora u istom tenantu ne mogu se dodati sa istim `email`.
- Ne smijete mijenjati `tenantId` povezan sa `Moderator`.

[inline-code-attrs-start title = 'Moderator PATCH cURL Primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH Struktura zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH Struktura odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Uključeno pri neuspjehu. **/
    reason?: string
}
[inline-code-end]