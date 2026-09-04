[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Dette API bruger paginering, leveret af parametrene `skip`, `limit`, `before` og `after`. AuditLogs returneres i sider på `5000` som standard, op til et maksimalt `limit` på `10000`, sorteret efter `when` og `id`. Siderne er store, fordi dette endpoint normalt bruges til at dump historik snarere end at bladre gennem den interaktivt.

Hvert `100` log, der returneres, har en kreditomkostning på `1`.

Som standard vil du modtage en liste med **de nyeste elementer først**. På denne måde kan du forespørge startende med `skip=0`, paginere indtil du finder den sidste post, du har forbrugt.

Alternativt kan du sortere ældste først og paginere indtil der ikke er flere poster.

Sortering kan udføres ved at sætte `order` til enten `ASC` eller `DESC`. Standard er `DESC`.

Forespørgsel efter dato er mulig via `before` og `after` som tidsstempler med millisekunder. `before` og `after` er IKKE inklusiv, og hver kan bruges alene.

## Find ud af hvad der skete med en person

Hver hændelse registrerer hvem der udførte den (`username`, `userId`, `ip`) og, separat, hvad den blev udført på. `targetLabel` er en menneskelig læsbar etiket for det objekt, for eksempel `jsmith (jsmith@example.com)`, og `targetId` er dens id. Brug `target` for en case‑insensitiv delstreng‑match på etiketten, når du kender en persons navn eller e‑mail men ikke deres id.

Sletninger fanger etiketten på tidspunktet for hændelsen, så en fjernet bruger eller moderator stadig kan identificeres efter den underliggende post er væk.

## Administrerede lejere

Hvis din lejer administrerer andre lejere, sæt `includeManagedTenants=true` for at returnere hændelser fra din lejer og hver lejer den administrerer i et enkelt svar. Hver returneret logs `tenantId` fortæller dig, hvilken lejer den kom fra.

[inline-code-attrs-start title = 'AuditLog cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'AuditLog svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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