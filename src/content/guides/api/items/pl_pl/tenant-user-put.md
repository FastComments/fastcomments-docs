[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia zastąpienie pojedynczego `TenantUser`.

Zastępowanie `TenantUser` ma następujące ograniczenia:

- Pole `signUpDate` nie może mieć daty w przyszłości.
- Wartość `locale` musi znajdować się na liście [Obsługiwane lokalizacje](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Pole `username` musi być unikalne w całym FastComments.com. Jeśli to stanowi problem, sugerujemy zamiast tego użycie SSO.
- Pole `email` musi być unikalne w całym FastComments.com. Jeśli to stanowi problem, sugerujemy zamiast tego użycie SSO.
- Nie można zaktualizować `tenantId` użytkownika.

Możemy zastąpić `TenantUser` w następujący sposób

[inline-code-attrs-start title = 'Przykład cURL zastąpienia TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania zastąpienia TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Gdy zmieniany jest email lub nazwa użytkownika, możesz ustawić to na true, aby również zaktualizować komentarze użytkownika. Spowoduje to podwojenie kosztu kredytów. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi zastąpienia TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]

---