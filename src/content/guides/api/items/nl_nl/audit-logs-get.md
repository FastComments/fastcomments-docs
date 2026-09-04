[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Deze API gebruikt paginering, geleverd door de parameters `skip`, `limit`, `before` en `after`. AuditLogs worden standaard in pagina's van `100` geretourneerd, tot een maximum `limit` van `200`, gesorteerd op `when` en `id`.

Elke `100` geretourneerde logs kost `1` credit.

Standaard ontvang je een lijst met **de nieuwste items eerst**. Op deze manier kun je pollen beginnend met `skip=0`, paginerend totdat je de laatste verbruikte record vindt.

Alternatief kun je sorteren van oud naar nieuw, en pagineren tot er geen records meer zijn.

Sorteren kan door `order` in te stellen op `ASC` of `DESC`. Standaard is `DESC`.

Opvragen op datum is mogelijk via `before` en `after` als timestamps met milliseconden. `before` en `after` zijn NIET inclusief, en elk kan afzonderlijk worden gebruikt.

## Finding what happened to a person

Elk evenement registreert wie het heeft uitgevoerd (`username`, `userId`, `ip`) en, apart, waarop het is uitgevoerd. `targetLabel` is een menselijk leesbaar label voor dat object, bijvoorbeeld `jsmith (jsmith@example.com)`, en `targetId` is de id. Gebruik `target` voor een hoofdletterongevoelige substring-match op het label wanneer je de naam of e‑mail van een persoon kent, maar niet de id.

Verwijderingen leggen het label vast op het moment van het evenement, zodat een verwijderde gebruiker of moderator nog steeds kan worden geïdentificeerd nadat het onderliggende record is verdwenen.

## Managed tenants

Als jouw tenant andere tenants beheert, stel `includeManagedTenants=true` in om evenementen van jouw tenant en elke tenant die het beheert in één respons te retourneren. De `tenantId` van elke geretourneerde log vertelt je van welke tenant deze afkomstig is.

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