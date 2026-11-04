[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućuje ažuriranje `Moderator` objekta putem `id`.

Ažuriranje `Moderator` objekta ima sljedeća ograničenja:

- Sljedeće vrijednosti se ne smiju navesti prilikom ažuriranja `Moderator` objekta:
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
- Kada je naveden `userId`, korisnik mora pripadati istom `tenantId` navedenom u query parametrima.
- Dva moderatora u istom tenantu ne mogu imati isti `email`.
- Ne možete promijeniti `tenantId` povezan s `Moderator` objektom.

[inline-code-attrs-start title = 'Primjer Moderator PATCH cURL zahtjeva'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Moderator PATCH zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Moderator PATCH odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]