[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ovaj API koristi paginaciju, koju pružaju parametri `skip`, `limit`, `before` i `after`. AuditLog zapisi se vraćaju u stranicama od `100` po zadanim postavkama, do maksimalnog `limit` od `200`, poredani po `when` i `id`.

Svaki `100` zapisa koji se vrati ima trošak od `1` kredita.

Prema zadanim postavkama, dobit ćete popis s **najnovijim stavkama prvo**. Na ovaj način možete dohvatiti podatke počevši od `skip=0`, paginirajući sve dok ne pronađete posljednji zapis koji ste obradili.

Alternativno, možete sortirati najstarije prvo i paginirati dok ne ostane više zapisa.

Sortiranje se može izvršiti postavljanjem `order` na `ASC` ili `DESC`. Zadano je `DESC`.

Upiti po datumu su mogući putem `before` i `after` kao vremenskih oznaka u milisekundama. `before` i `after` NISU inkluzivni i svaki se može koristiti samostalno.

## Finding what happened to a person

Svaki događaj bilježi tko ga je izvršio (`username`, `userId`, `ip`) i, odvojeno, na čemu je izvršen. `targetLabel` je čitljiva oznaka za taj objekt, na primjer `jsmith (jsmith@example.com)`, a `targetId` je njegov ID. Koristite `target` za podudaranje podstringa neosjetljivo na veličinu slova na oznaci kada znate ime ili e‑mail osobe, ali ne i njen ID.

Brisanja bilježe oznaku u trenutku događaja, tako da se uklonjeni korisnik ili moderator i dalje mogu identificirati nakon što je osnovni zapis uklonjen.

## Managed tenants

Ako vaš najam upravlja drugim najmovima, postavite `includeManagedTenants=true` kako biste vratili događaje iz vašeg najma i svakog najma koji on upravlja u jednom odgovoru. Svaki vraćeni zapis ima `tenantId` koji vam govori iz kojeg najma potječe.

[inline-code-attrs-start title = 'AuditLog cURL primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog struktura zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Samo događaji izvršeni od strane ovog korisničkog imena. **/
    username?: string
    /** Samo događaji s ove IP adrese. **/
    ip?: string
    /** Samo događaji ove vrste. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Samo događaji za ovaj resurs, npr. User ili Moderator. **/
    resourceName?: string
    /** Samo događaji čiji je zahvaćeni objekt ima ovaj ID. **/
    targetId?: string
    /** Podudaranje podstringa neosjetljivo na veličinu slova na oznaci zahvaćenog objekta. **/
    target?: string
    /** Također vrati događaje iz najma koje ovaj najam upravlja. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog struktura odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Uključeno pri neuspjehu. **/
    reason?: string
    /** Zapisnici! **/
    auditLogs: AuditLog[]
}
[inline-code-end]