[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt biedt de mogelijkheid om een `Moderator` bij te werken via `id`.

Het bijwerken van een `Moderator` kent de volgende beperkingen:

- De volgende waarden mogen niet worden opgegeven bij het bijwerken van een `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Als een `userId` is opgegeven, moet die gebruiker bestaan.
- Als een `userId` is opgegeven, moet deze behoren tot dezelfde `tenantId` die in de queryparameters is opgegeven.
- Twee moderators in dezelfde tenant kunnen niet worden toegevoegd met hetzelfde `email`.
- U mag de `tenantId` die aan een `Moderator` is gekoppeld niet wijzigen.

[inline-code-attrs-start title = 'Moderator PATCH cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH Antwoordstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij falen. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Opgenomen bij falen. **/
    reason?: string
}
[inline-code-end]