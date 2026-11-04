[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Ta trasa umożliwia zaproszenie pojedynczego `Moderatora`.

Aby wysłać e-mail z zaproszeniem do `Moderatora`, obowiązują następujące ograniczenia:
- `Moderator` musi już istnieć.
- `fromName` nie może być dłuższe niż `100 znaków`.

**Uwagi:**
- Jeśli użytkownik z podanym adresem e-mail już istnieje, zostanie zaproszony do moderowania komentarzy Twojej tenanta.
- Jeśli użytkownik z podanym adresem e-mail **nie istnieje**, link zapraszający przeprowadzi go przez proces tworzenia konta.
- Zaproszenie wygaśnie po `30 dniach`.

Możemy utworzyć `Moderatora` dla użytkownika, którego znamy tylko adres e-mail:

[inline-code-attrs-start title = 'Przykład zaproszenia Moderatora za pomocą cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Spowoduje to wysłanie e-maila w stylu `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura żądania zaproszenia Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** E-mail wysłany do użytkownika będzie wyglądał, jakby był wysłany od tej nazwy. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi zaproszenia Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku błędu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Dołączane w przypadku błędu. **/
    reason?: string
}
[inline-code-end]