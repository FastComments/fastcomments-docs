[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ova ruta pruža mogućnost dodavanja jednog `Moderator`.

Kreiranje `Moderator` ima sledeća ograničenja:

- Uvek je potrebno navesti `name` i `email`. `userId` je opciono.
- Sledeće vrednosti ne smeju biti prosleđene pri kreiranju `Moderator`:
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
- Kada je naveden `userId`, on mora pripadati istom `tenantId` koji je naveden u parametrima upita.
- Dva moderatora u istom tenantu ne mogu biti dodata sa istim `email`.

Možemo kreirati `Moderator` za korisnika o kojem znamo samo email:

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje Moderator-a putem email-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Ili možemo kreirati `Moderator` za korisnika koji pripada našem tenantu, da bismo pratili njihove statistike moderacije:

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje Moderator-a za korisnika tenant-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteva za kreiranje Moderator-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora pri kreiranju Moderator-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    moderator?: Moderator; // Vraćamo kompletno kreiranog moderatora pri uspehu.
}
[inline-code-end]