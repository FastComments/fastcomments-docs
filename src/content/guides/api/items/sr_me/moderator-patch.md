[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ova API krajnja tačka omogućava ažuriranje `Moderator` po `id`.

Ažuriranje `Moderator`-a ima sljedeća ograničenja:

- Sljedeće vrijednosti ne smiju biti proslijeđene pri ažuriranju `Moderator`-a:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Kada je specificiran `userId`, taj korisnik mora postojati.
- Kada je specificiran `userId`, on mora pripadati istom `tenantId` navedenom u query parametrima.
- Dva moderatora u istom tenantu ne mogu imati isti `email`.
- Ne smijete mijenjati `tenantId` povezan s `Moderator`-om.

[inline-code-attrs-start title = 'Primjer PATCH cURL zahtjeva za Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahtjeva za Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH odgovora za Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Uključeno pri neuspjehu. **/
    reason?: string
}
[inline-code-end]