[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Deze API gebruikt paginering, geleverd door de `skip`, `limit`, `before` en `after` parameters. AuditLogs worden standaard geretourneerd in pagina's van `5000`, tot een maximale `limit` van `10000`, gesorteerd op `when` en `id`. De pagina's zijn groot omdat dit eindpunt meestal wordt gebruikt om de geschiedenis te dumpen in plaats van er interactief doorheen te pagineren.

Elke `100` logs die worden geretourneerd kost `1` credit.

Standaard ontvang je een lijst met **de nieuwste items eerst**. Op deze manier kun je pollen beginnend met `skip=0`, paginerend totdat je het laatste record vindt dat je hebt verbruikt.

Alternatief kun je sorteren van oud naar nieuw, en pagineren totdat er geen records meer zijn.

Sorteren kan worden gedaan door `order` in te stellen op `ASC` of `DESC`. Standaard is `DESC`.

Queryen op datum is mogelijk via `before` en `after` als timestamps met milliseconden. `before` en `after` zijn NIET inclusief, en elk kan afzonderlijk worden gebruikt.

## Vinden wat er met een persoon is gebeurd

Elk evenement registreert wie het heeft uitgevoerd (`username`, `userId`, `ip`) en, apart, waarop het is uitgevoerd. `targetLabel` is een menselijk leesbaar label voor dat object, bijvoorbeeld `jsmith (jsmith@example.com)`, en `targetId` is de id ervan. Gebruik `target` voor een case‑insensitieve substring match op het label wanneer je de naam of e‑mail van een persoon kent maar niet hun id.

Verwijderingen leggen het label vast op het moment van het evenement, zodat een verwijderde gebruiker of moderator nog steeds kan worden geïdentificeerd nadat het onderliggende record is verdwenen.

## Beheerde tenants

Als jouw tenant andere tenants beheert, stel `includeManagedTenants=true` in om evenementen van jouw tenant en elke tenant die het beheert in één respons te retourneren. De `tenantId` van elk geretourneerd log vertelt je van welke tenant het afkomstig is.

[inline-code-attrs-start title = 'AuditLog cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Defaults to 5000. **/
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

[inline-code-attrs-start title = 'AuditLog Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---