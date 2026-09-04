[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ta API uporablja paginacijo, ki jo omogočajo parametri `skip`, `limit`, `before` in `after`. AuditLogi se privzeto vrnejo v straneh po `5000` zapisov, do največjega `limit`a `10000`, urejeni po `when` in `id`. Strani so velike, ker se ta končna točka običajno uporablja za izvoz zgodovine, namesto da bi se po njej interaktivno pomikal.

Vsakih `100` vrnjenih zapisov ima strošek kredita `1`.

Privzeto boste prejeli seznam z **najnovejšimi elementi najprej**. Na ta način lahko poizvedujete z začetkom `skip=0` in paginirate, dokler ne najdete zadnjega zapisa, ki ste ga porabili.

Alternativno lahko razvrstite po najstarejših najprej in paginirate, dokler ne ostane več zapisov.

Razvrščanje lahko izvedete z nastavitvijo `order` na `ASC` ali `DESC`. Privzeto je `DESC`.

Poizvedovanje po datumu je mogoče prek `before` in `after` kot časovnih žigov v milisekundah. `before` in `after` NISTA vključena, in katerikoli se lahko uporabi samostojno.

## Finding what happened to a person

Vsak dogodek zabeleži, kdo ga je izvedel (`username`, `userId`, `ip`) in ločeno, na čem je bil izveden. `targetLabel` je človeško berljiva oznaka za ta objekt, na primer `jsmith (jsmith@example.com)`, `targetId` pa je njegov ID. Uporabite `target` za neobčutljivo na velikost črk delno ujemanje podniza v oznaki, ko poznate ime ali e‑naslov osebe, vendar ne njen ID.

Brisanja zajamejo oznako v času dogodka, zato je odstranjenega uporabnika ali moderatorja še vedno mogoče identificirati, ko je osnovni zapis izginil.

## Managed tenants

Če vaš najemnik upravlja druge najemnike, nastavite `includeManagedTenants=true`, da vrnete dogodke iz vašega najemnika in vseh najemnikov, ki jih upravlja, v enem odgovoru. Vsak vrnjeni zapis ima `tenantId`, ki pove, iz katerega najemnika je.

[inline-code-attrs-start title = 'Primer cURL zahteve za AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Največ 10000. Privzeto 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Samo dogodki, ki jih je izvedel ta uporabniško ime. **/
    username?: string
    /** Samo dogodki s tega IP naslova. **/
    ip?: string
    /** Samo dogodki tega tipa. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Samo dogodki za ta vir, npr. Uporabnik ali Moderator. **/
    resourceName?: string
    /** Samo dogodki, katerih prizadeti objekt ima ta ID. **/
    targetId?: string
    /** Neobčutljivo na velikost črk delno ujemanje podniza v oznaki prizadetega objekta. **/
    target?: string
    /** Vrni tudi dogodke iz najemnikov, ki jih ta najemnik upravlja. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Dnevniki! **/
    auditLogs: AuditLog[]
}
[inline-code-end]