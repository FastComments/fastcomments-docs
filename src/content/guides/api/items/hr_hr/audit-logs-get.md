[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ovaj API koristi paginaciju, koju pružaju parametri `skip`, `limit`, `before` i `after`. AuditLog zapisi se vraćaju u stranicama od `1000` po zadanim postavkama, do maksimalnog `limit` od `10000`, poredani po `when` i `id`. Stranice su velike jer se ovaj krajnji punkt obično koristi za izvoz povijesti, a ne za interaktivno listanje.

Svaki `100` vraćenih zapisa košta `1` kredit.

Prema zadanim postavkama, dobit ćete popis s **najnovijim stavkama prvo**. Na ovaj način možete povlačiti podatke počevši od `skip=0`, paginirajući sve dok ne pronađete posljednji zapis koji ste obradili.

Alternativno, možete sortirati najstarije prvo i paginirati sve dok ne ostane više zapisa.

Sortiranje se može izvršiti postavljanjem `order` na `ASC` ili `DESC`. Zadano je `DESC`.

Upiti po datumu mogu se izvršiti putem `before` i `after` kao vremenskih oznaka u milisekundama. `before` i `after` NISU inkluzivni i svaki se može koristiti samostalno.

## Pronalaženje što se dogodilo osobi

Svaki događaj bilježi tko ga je izvršio (`username`, `userId`, `ip`) i, odvojeno, na čemu je izvršen. `targetLabel` je čitljiva oznaka za taj objekt, na primjer `jsmith (jsmith@example.com)`, a `targetId` je njegov ID. Koristite `target` za podudaranje podstringa neosjetljivo na veličinu slova na oznaci kada znate ime ili e‑mail osobe, ali ne i njen ID.

Brisanja bilježe oznaku u trenutku događaja, tako da se uklonjeni korisnik ili moderator i dalje može identificirati nakon što je osnovni zapis izbrisan.

## Upravljani najmodavci

Ako vaš najmodavac upravlja drugim najmodavcima, postavite `includeManagedTenants=true` kako biste vratili događaje iz vašeg najmodavca i svih najmodavaca koje on upravlja u jednom odgovoru. Svaki vraćeni zapis ima `tenantId` koji vam govori iz kojeg najmodavca potječe.

[inline-code-attrs-start title = 'AuditLog cURL primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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