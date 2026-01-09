[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Ta ścieżka umożliwia dodanie pojedynczego `TenantUser`.

Tworzenie `TenantUser` ma następujące ograniczenia:

- Wymagane jest podanie `username`.
- Wymagane jest podanie `email`.
- `signUpDate` nie może być w przyszłości.
- `locale` musi być na liście [Obsługiwane lokalizacje](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` musi być unikalny na całym FastComments.com. Jeśli stanowi to problem, sugerujemy użycie SSO.
- `email` musi być unikalny na całym FastComments.com. Jeśli stanowi to problem, sugerujemy użycie SSO.
- Nie można utworzyć więcej użytkowników najemcy niż zdefiniowano w `maxTenantUsers` w Twoim pakiecie. 

Możemy utworzyć `TenantUser` w następujący sposób

[inline-code-attrs-start title = 'Przykład cURL tworzenia TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania tworzenia TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tworzenia TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    tenantUser?: TenantUser; // W przypadku powodzenia zwracamy w pełni utworzonego TenantUser.
}
[inline-code-end]

---