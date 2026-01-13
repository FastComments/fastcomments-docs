[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućava dodavanje jednog `Moderator`.

Kreiranje `Moderator` ima sljedeća ograničenja:

- `name` i `email` uvijek moraju biti navedeni. `userId` je opciono.
- Sljedeće vrijednosti se ne smiju navoditi prilikom kreiranja `Moderator`:
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
- Kada je `userId` naveden, mora pripadati istom `tenantId` navedenom u parametrima upita.
- Dva moderatora u istom tenantu ne mogu imati isti `email`.

Možemo kreirati `Moderator` za korisnika za kojeg znamo samo email:

[inline-code-attrs-start title = 'Primjer kreiranja Moderator-a putem e-pošte (cURL)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Primjer kreiranja Moderator-a za korisnika iz tenanta (cURL)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtjeva za kreiranje Moderator-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    moderator?: Moderator; // Vraćamo kompletno kreiranog moderatora u slučaju uspjeha.
}
[inline-code-end]

---