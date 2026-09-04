[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ta API uporablja paginacijo, ki jo zagotavljajo parametri `skip`, `limit`, `before` in `after`. AuditLogi se privzeto vrnejo v straneh po `1000` zapisov, do največjega `limit`a `10000`, urejeni po `when` in `id`. Strani so velike, ker se ta končna točka običajno uporablja za izpis zgodovine, namesto da bi se po njej interaktivno pomikal.

Vsakih `100` vrnjenih zapisov ima strošek kredita `1`.

Privzeto boste prejeli seznam z **najnovejšimi elementi na začetku**. Na ta način lahko poizvedujete z začetkom `skip=0` in paginirate, dokler ne najdete zadnjega zapisa, ki ste ga porabili.

Alternativno lahko razvrstite po najstarejših najprej in paginirate, dokler ne ostane več zapisov.

Razvrščanje lahko izvedete z nastavitvijo `order` na `ASC` ali `DESC`. Privzeto je `DESC`.

Poizvedovanje po datumu je mogoče prek `before` in `after` kot časovnih žigov v milisekundah. `before` in `after` NISTA vključena, in katerikoli se lahko uporabi samostojno.

## Iskanje, kaj se je zgodilo osebi

Vsak dogodek zabeleži, kdo ga je izvedel (`username`, `userId`, `ip`) in ločeno, na čem je bil izveden. `targetLabel` je človeško berljiva oznaka za ta objekt, na primer `jsmith (jsmith@example.com)`, `targetId` pa je njegov ID. Uporabite `target` za neobčutljivo na velikost črk delno ujemanje podniza v oznaki, ko poznate ime ali e‑naslov osebe, vendar ne njen ID.

Brisanja zajamejo oznako v času dogodka, zato je odstranjenega uporabnika ali moderatorja še vedno mogoče identificirati, ko je osnovni zapis izginil.

## Upravljani najemniki

Če vaš najemnik upravlja druge najemnike, nastavite `includeManagedTenants=true`, da v enem odgovoru vrnete dogodke iz vašega najemnika in vseh najemnikov, ki jih upravlja. Vsak vrnjeni zapis ima `tenantId`, ki pove, iz katerega najemnika je.

[inline-code-attrs-start title = 'Primer cURL za AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Defaults to 1000. **/
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

[inline-code-attrs-start title = 'Struktura odgovora AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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