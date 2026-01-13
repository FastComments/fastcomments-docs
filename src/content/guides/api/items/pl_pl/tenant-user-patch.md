[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ta trasa zapewnia możliwość aktualizacji pojedynczego `TenantUser`.

Aktualizacja `TenantUser` ma następujące ograniczenia:

- `signUpDate` nie może być w przyszłości.
- `locale` musi znajdować się na liście [Obsługiwane lokalizacje](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` musi być unikalny w całym serwisie FastComments.com. Jeśli to problem, sugerujemy użycie SSO.
- `email` musi być unikalny w całym serwisie FastComments.com. Jeśli to problem, sugerujemy użycie SSO.
- Nie można zaktualizować `tenantId` użytkownika.

Możemy utworzyć `TenantUser` w następujący sposób

[inline-code-attrs-start title = 'Przykład żądania cURL aktualizacji TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania aktualizacji TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Gdy email lub nazwa użytkownika zostaną zmienione, możesz ustawić to na true, aby również zaktualizować komentarze użytkownika. Spowoduje to podwojenie kosztu w kredytach. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi aktualizacji TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]