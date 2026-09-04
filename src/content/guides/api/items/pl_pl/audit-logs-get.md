[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

To API używa paginacji, udostępnianej przez parametry `skip`, `limit`, `before` i `after`. AuditLogi są zwracane w stronach po `1000` domyślnie, do maksymalnego `limit` wynoszącego `10000`, uporządkowane według `when` i `id`. Strony są duże, ponieważ ten endpoint jest zazwyczaj używany do zrzutu historii, a nie do interaktywnego przewijania.

Każde `100` zwróconych logów kosztuje `1` kredyt.

Domyślnie otrzymasz listę z **najnowszymi elementami jako pierwsze**. W ten sposób możesz pollować zaczynając od `skip=0`, paginując aż znajdziesz ostatni rekord, który już pobrałeś.

Alternatywnie możesz sortować od najstarszych, i paginować aż nie będzie już więcej rekordów.

Sortowanie można wykonać ustawiając `order` na `ASC` lub `DESC`. Domyślnie jest `DESC`.

Zapytania według daty są możliwe przy użyciu `before` i `after` jako znaczników czasu w milisekundach. `before` i `after` NIE są inkluzywne i każde z nich może być użyte samodzielnie.

## Znalezienie, co stało się z osobą

Każde zdarzenie rejestruje, kto je wykonał (`username`, `userId`, `ip`) oraz, osobno, na czym zostało wykonane. `targetLabel` to czytelna etykieta tego obiektu, na przykład `jsmith (jsmith@example.com)`, a `targetId` to jego identyfikator. Użyj `target` do dopasowania podciągu (bez uwzględniania wielkości liter) w etykiecie, gdy znasz imię lub e‑mail osoby, ale nie jej identyfikator.

Usunięcia przechwytują etykietę w momencie zdarzenia, więc usunięty użytkownik lub moderator może być nadal zidentyfikowany po usunięciu pierwotnego rekordu.

## Zarządzane najemcy

Jeśli Twój tenant zarządza innymi tenantami, ustaw `includeManagedTenants=true`, aby zwrócić zdarzenia z Twojego tenantu oraz ze wszystkich tenantów, które on zarządza, w jednej odpowiedzi. `tenantId` każdego zwróconego logu informuje, z którego tenantu pochodzi.

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
    /** Maksymalnie 10000. Domyślnie 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Tylko zdarzenia wykonane przez tę nazwę użytkownika. **/
    username?: string
    /** Tylko zdarzenia z tego adresu IP. **/
    ip?: string
    /** Tylko zdarzenia tego typu. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Tylko zdarzenia dla tego zasobu, np. Użytkownik lub Moderator. **/
    resourceName?: string
    /** Tylko zdarzenia, których dotknięty obiekt ma ten identyfikator. **/
    targetId?: string
    /** Dopasowanie podciągu (bez uwzględniania wielkości liter) w etykiecie dotkniętego obiektu. **/
    target?: string
    /** Również zwróć zdarzenia z tenantów, które ten tenant zarządza. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    /** Logi! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---