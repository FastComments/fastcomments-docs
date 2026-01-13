[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava dodavanje jednog `Moderator`.

Kreiranje `Moderator` objekta ima sledeća ograničenja:

- Uvek je potrebno navesti `name` i `email`. `userId` je opciono.
- Sledeće vrednosti ne smeju biti prosleđene prilikom kreiranja `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Kada je `userId` naveden, taj korisnik mora postojati.
- Kada je `userId` naveden, mora pripadati istom `tenantId` koji je naveden u parametrima upita.
- Dva moderatora u istom tenantu ne mogu biti dodata sa istim `email`.

Možemo kreirati `Moderator` za korisnika za kojeg znamo samo `email`:

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje Moderatora putem e-pošte'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Ili možemo kreirati `Moderator` za korisnika koji pripada našem tenantu, kako bismo pratili njihove statistike moderacije:

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje Moderatora za korisnika koji pripada tenantu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteva za kreiranje Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Uključeno u slučaju greške. **/
    reason?: string
    moderator?: Moderator; // U slučaju uspeha vraćamo kompletno kreiranog moderatora.
}
[inline-code-end]

---