[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Ta trasa udostępnia możliwość zaproszenia pojedynczego `Moderator`.

The following restrictions exist to send an invite email to a `Moderator`:
- The `Moderator` must already exist.
- The `fromName` may not be longer than `100 characters`.

**Notes:**
- Jeśli użytkownik z podanym adresem e-mail już istnieje, zostanie zaproszony do moderowania komentarzy twojego tenanta.
- Jeśli użytkownik z podanym adresem e-mail **nie istnieje** link zaproszenia poprowadzi go przez proces tworzenia konta.
- Zaproszenie wygaśnie po `30 days`.

Możemy utworzyć `Moderator` dla użytkownika, którego znamy tylko adres e-mail:

[inline-code-attrs-start title = 'Przykład żądania cURL: Zaproszenie moderatora'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

To wyśle e-mail podobny do `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura żądania zaproszenia moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** E-mail wysyłany do użytkownika będzie wyglądał, jakby był wysłany z tej nazwy. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi zaproszenia moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]