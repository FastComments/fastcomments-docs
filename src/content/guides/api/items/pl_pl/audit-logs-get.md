[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

To API używa paginacji, zapewnianej przez parametry `skip`, `before` i `after`. AuditLogs są zwracane stronami po `1000`, posortowane według `when` i `id`.

Pobranie każdej partii `1000` logów kosztuje `10` kredytów.

Domyślnie otrzymasz listę z **najnowszymi elementami na początku**. Dzięki temu możesz odpytywać zaczynając od `skip=0`, paginując aż znajdziesz ostatni rekord, który przetworzyłeś.

Alternatywnie możesz sortować od najstarszych i paginować, aż nie będzie więcej rekordów.

Sortowanie można ustawić przez przypisanie wartości `order` na `ASC` lub `DESC`. Domyślną wartością jest `ASC`.

Zapytania według daty są możliwe przy użyciu `before` i `after` jako znaczników czasu z milisekundami. `before` i `after` NIE są włączające.

[inline-code-attrs-start title = 'Przykład żądania cURL AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    /** Logi! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---