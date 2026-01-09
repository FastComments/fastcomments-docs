[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ten punkt końcowy API umożliwia zaktualizowanie `Moderator` za pomocą `id`.

Aktualizacja `Moderator` ma następujące ograniczenia:

- Następujące wartości nie mogą być podane podczas aktualizacji `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Jeżeli zostanie podane `userId`, użytkownik musi istnieć.
- Jeżeli zostanie podane `userId`, musi należeć do tego samego `tenantId`, który został podany w parametrach zapytania.
- Dwóch moderatorów w tym samym tenant nie może mieć tego samego `email`.
- Nie można zmienić `tenantId` powiązanego z `Moderator`.

[inline-code-attrs-start title = 'Przykład żądania cURL (PATCH Moderator)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania PATCH Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi PATCH Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]