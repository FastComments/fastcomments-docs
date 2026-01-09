[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Ta trasa umożliwia wysłanie linku logowania do pojedynczego `TenantUser`.

Przydatne przy masowym tworzeniu użytkowników, aby nie instruować ich, jak logować się do FastComments.com. Spowoduje to po prostu wysłanie im "magicznego linku" do logowania, który wygasa po `30 days`.

Istnieją następujące ograniczenia dotyczące wysyłania linku logowania do `TenantUser`:
- `TenantUser` musi już istnieć.
- Musisz mieć dostęp do zarządzania `Tenant`, do którego należy `TenantUser`.

Możemy wysłać link logowania do `TenantUser` w następujący sposób:

[inline-code-attrs-start title = 'Przykład żądania cURL: link logowania dla TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

To spowoduje wysłanie wiadomości e-mail takiej jak `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura żądania linku logowania TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi linku logowania TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku błędu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Dołączane w przypadku błędu. **/
    reason?: string
}
[inline-code-end]

---