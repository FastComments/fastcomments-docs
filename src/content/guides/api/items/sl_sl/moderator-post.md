[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ta ruta omogoča dodajanje enega samega `Moderator`.

Ustvarjanje `Moderator` ima naslednje omejitve:

- Vedno morate navesti `name` in `email`. `userId` je izbiren.
- Pri ustvarjanju `Moderator` ni dovoljeno navesti naslednjih vrednosti:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Če je naveden `userId`, ta uporabnik mora obstajati.
- Če je naveden `userId`, mora pripadati istemu `tenantId`, navedenemu v parametrkih poizvedbe.
- Dva moderatorja v istem najemniku ne moreta imeti istega `email`.

Ustvarimo lahko `Moderator` za uporabnika, o katerem poznamo samo e-poštni naslov:

[inline-code-attrs-start title = 'Ustvarjanje Moderatorja prek e-pošte — primer cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Lahko pa ustvarimo `Moderator` za uporabnika, ki pripada našemu najemniku, da spremljamo njegove statistike moderiranja:

[inline-code-attrs-start title = 'Ustvarjanje Moderatorja prek uporabnika najemnika — primer cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtevka za ustvarjanje Moderatorja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ustvarjanje Moderatorja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Vključeno ob neuspehu. **/
    reason?: string
    moderator?: Moderator; // Ob uspehu vrnemo popolnoma ustvarjenega moderatorja.
}
[inline-code-end]