[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ten endpoint umożliwia dodanie pojedynczego `Moderator`.

Tworzenie `Moderator` ma następujące ograniczenia:

- Zawsze należy podać `name` i `email`. `userId` jest opcjonalne.
- Następujące wartości nie mogą być podawane podczas tworzenia `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Jeśli podano `userId`, użytkownik musi istnieć.
- Jeśli podano `userId`, musi należeć do tego samego `tenantId` podanego w parametrach zapytania.
- Dwóch moderatorów w tym samym tenant nie może zostać dodanych z tym samym `email`.

Możemy utworzyć `Moderator` dla użytkownika, którego znamy tylko adres e-mail:

[inline-code-attrs-start title = 'Przykład cURL: utworzenie Moderator na podstawie adresu e-mail'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Lub możemy utworzyć `Moderator` dla użytkownika, który należy do naszego tenant, aby śledzić ich statystyki moderacji:

[inline-code-attrs-start title = 'Przykład cURL: utworzenie Moderator dla użytkownika należącego do tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura żądania tworzenia Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tworzenia Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    moderator?: Moderator; // Zwracamy kompletnie utworzonego moderatora w przypadku powodzenia.
}
[inline-code-end]