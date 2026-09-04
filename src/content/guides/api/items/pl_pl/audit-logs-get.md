[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

To API używa paginacji, udostępnianej przez parametry `skip`, `limit`, `before` i `after`. AuditLogi są zwracane w stronach po `100` domyślnie, do maksymalnego `limit` wynoszącego `200`, uporządkowane według `when` i `id`.

Każde `100` zwróconych logów kosztuje `1` kredyt.

Domyślnie otrzymasz listę z **najnowszymi elementami najpierw**. Dzięki temu możesz pobierać zaczynając od `skip=0`, paginując aż znajdziesz ostatni rekord, który został zużyty.

Alternatywnie możesz sortować od najstarszych, i paginować aż nie będzie już więcej rekordów.

Sortowanie można wykonać ustawiając `order` na `ASC` lub `DESC`. Domyślnie jest `DESC`.

Zapytania według daty są możliwe przy użyciu `before` i `after` jako znaczników czasu w milisekundach. `before` i `after` NIE są inkluzywne i każdy z nich może być użyty samodzielnie.

## Znalezienie, co stało się z osobą

Każde zdarzenie rejestruje, kto je wykonał (`username`, `userId`, `ip`) oraz, osobno, na czym zostało wykonane. `targetLabel` to czytelna etykieta tego obiektu, na przykład `jsmith (jsmith@example.com)`, a `targetId` to jego identyfikator. Użyj `target` do dopasowania podciągu (bez rozróżniania wielkości liter) w etykiecie, gdy znasz imię lub e‑mail osoby, ale nie jej identyfikator.

Usunięcia zapisują etykietę w momencie zdarzenia, więc usunięty użytkownik lub moderator może być nadal zidentyfikowany po usunięciu pierwotnego rekordu.

## Zarządzane najemcy

Jeśli Twój najemca zarządza innymi najemcami, ustaw `includeManagedTenants=true`, aby zwrócić zdarzenia z Twojego najemcy oraz ze wszystkich najemców, którymi zarządza, w jednej odpowiedzi. `tenantId` każdego zwróconego logu informuje, z którego najemcy pochodzi.

[inline-code-attrs-start title = 'Przykład cURL AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]