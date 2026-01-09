[api-resource-header-start name = 'SSOUser'; route = 'PATCH /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia aktualizację pojedynczego użytkownika SSO.

[inline-code-attrs-start title = 'Przykład żądania cURL aktualizacji SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "notfordperfect"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania aktualizacji SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Gdy email lub nazwa użytkownika zostaną zmienione, możesz ustawić to na true, aby również zaktualizować komentarze użytkownika. Spowoduje to podwojenie kosztu kredytów. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi aktualizacji SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-does-not-exist'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    user?: SSOUser; // Zwracamy pełnego zaktualizowanego użytkownika w przypadku powodzenia.
}
[inline-code-end]


---