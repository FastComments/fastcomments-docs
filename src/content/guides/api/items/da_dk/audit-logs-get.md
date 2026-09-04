[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Dette API bruger paginering, leveret af parametrene `skip`, `limit`, `before` og `after`. AuditLogs returneres i sider på `100` som standard, op til et maksimalt `limit` på `200`, sorteret efter `when` og `id`.

Hver `100` logposter, der returneres, har en kreditomkostning på `1`.

Som standard vil du modtage en liste med **de nyeste elementer først**. På denne måde kan du forespørge startende med `skip=0` og paginere, indtil du finder den sidste post, du har forbrugt.

Alternativt kan du sortere ældste-først og paginere, indtil der ikke er flere poster.

Sortering kan udføres ved at sætte `order` til enten `ASC` eller `DESC`. Standard er `DESC`.

Forespørgsel efter dato er mulig via `before` og `after` som tidsstempler med millisekunder. `before` og `after` er IKKE inklusiv, og hver kan bruges alene.

## Finding what happened to a person

Hver hændelse registrerer, hvem der udførte den (`username`, `userId`, `ip`) og, separat, hvad den blev udført på. `targetLabel` er en menneskelig læsbar etiket for det objekt, for eksempel `jsmith (jsmith@example.com)`, og `targetId` er dens id. Brug `target` for en case‑insensitiv delstrengssammenligning på etiketten, når du kender en persons navn eller e‑mail, men ikke deres id.

Sletninger fanger etiketten på tidspunktet for hændelsen, så en fjernet bruger eller moderator stadig kan identificeres, efter den underliggende post er væk.

## Managed tenants

Hvis din lejer administrerer andre lejere, skal du sætte `includeManagedTenants=true` for at returnere hændelser fra din lejer og alle lejere, administreret, i ét svar. Hver returneret logs `tenantId` fortæller dig, hvilken lejer den kom fra.

[inline-code-attrs-start title = 'AuditLog cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Kun hændelser udført af dette brugernavn. **/
    username?: string
    /** Kun hændelser fra denne IP-adresse. **/
    ip?: string
    /** Kun hændelser af denne type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Kun hændelser for denne ressource, f.eks. Bruger eller Moderator. **/
    resourceName?: string
    /** Kun hændelser, hvor det berørte objekt har dette id. **/
    targetId?: string
    /** Case-insensitiv delstrengssammenligning på det berørte objekts etiket. **/
    target?: string
    /** Returnér også hændelser fra lejere, som denne lejer administrerer. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Inkluderet ved fejl. **/
    reason?: string
    /** Loggene! **/
    auditLogs: AuditLog[]
}
[inline-code-end]