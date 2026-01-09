[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje dodavanje jednog `Moderator`.

Kreiranje `Moderator` ima sljedeća ograničenja:

- Obavezno je navesti `name` i `email`. `userId` je neobavezno.
- Sljedeće vrijednosti se ne smiju navoditi pri stvaranju `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Ako je naveden `userId`, taj korisnik mora postojati.
- Ako je naveden `userId`, mora pripadati istom `tenantId` navedenom u parametrima upita.
- Dva moderatora unutar istog tenanta ne mogu biti dodana s istim `email`.

Možemo stvoriti `Moderator` za korisnika za kojeg znamo samo email:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za stvaranje Moderator-a putem e-pošte'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Ili možemo stvoriti `Moderator` za korisnika koji pripada našem tenantu, kako bismo pratili njihove statistike moderiranja:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za stvaranje Moderator-a putem korisnika iz tenanta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtjeva za stvaranje Moderator-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za stvaranje Moderator-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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