[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia aktualizację pojedynczego użytkownika SSO.

[inline-code-attrs-start title = 'Przykład cURL aktualizacji SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

W tym przykładzie określamy `groupIds` dla kontroli dostępu, ale jest to opcjonalne.

[inline-code-attrs-start title = 'Struktura żądania aktualizacji SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Gdy e-mail lub nazwa użytkownika zostaną zmienione, możesz ustawić to na true, aby również zaktualizować komentarze użytkownika. Spowoduje to podwojenie kosztu w kredytach. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi aktualizacji SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    user?: SSOUser; // Zwracamy zaktualizowanego użytkownika w przypadku powodzenia.
}
[inline-code-end]