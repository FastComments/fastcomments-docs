[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ta API končna točka omogoča posodobitev `Moderator` po `id`.

Posodabljanje `Moderator` ima naslednje omejitve:

- Pri posodabljanju `Moderator` ni dovoljeno posredovati naslednjih vrednosti:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Če je naveden `userId`, mora ta uporabnik obstajati.
- Če je naveden `userId`, mora pripadati istemu `tenantId`, navedenemu v parametrih poizvedbe.
- Dva moderatorja v istem najemniku ne moreta biti dodana z istim `email`.
- Ne smete spremeniti `tenantId`, povezanega z `Moderator`.

[inline-code-attrs-start title = 'Primer cURL zahteve za Moderator PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve PATCH za Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora PATCH za Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Vključeno v primeru neuspeha. **/
    reason?: string
}
[inline-code-end]