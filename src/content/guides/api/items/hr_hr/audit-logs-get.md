[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ovaj API koristi paginaciju, koju pružaju parametri `skip`, `limit`, `before` i `after`. AuditLog zapisi se vraćaju u stranicama od `5000` po zadanim postavkama, do maksimalnog `limit` od `10000`, poredani po `when` i `id`. Stranice su velike jer se ovaj krajnji punkt obično koristi za izvoz povijesti, a ne za interaktivno listanje.

Svaki `100` vraćenih zapisa košta `1` kredit.

Prema zadanim postavkama, dobit ćete popis s **najnovijim stavkama prvo**. Na ovaj način možete periodično dohvaćati počevši od `skip=0`, paginirajući sve dok ne pronađete posljednji zapis koji ste obradili.

Alternativno, možete sortirati od najstarijeg prema najnovijem i paginirati sve dok ne ostane više zapisa.

Sortiranje se može izvršiti postavljanjem `order` na `ASC` ili `DESC`. Zadana vrijednost je `DESC`.

Upiti po datumu mogu se izvesti putem `before` i `after` kao vremenskih oznaka u milisekundama. `before` i `after` NISU inkluzivni i svaki se može koristiti samostalno.

## Pronalaženje što se dogodilo osobi

Svaki događaj bilježi tko ga je izvršio (`username`, `userId`, `ip`) i, odvojeno, na čemu je izvršen. `targetLabel` je čitljiva oznaka za taj objekt, na primjer `jsmith (jsmith@example.com)`, a `targetId` je njegov ID. Koristite `target` za podudaranje podstringa neosjetljivog na veličinu slova na oznaci kada znate ime ili e‑mail osobe, ali ne i njen ID.

Brisanja bilježe oznaku u trenutku događaja, tako da se uklonjeni korisnik ili moderator i dalje mogu identificirati nakon što je osnovni zapis uklonjen.

## Upravljani najmodavci

Ako vaš najmodavac upravlja drugim najmodavcima, postavite `includeManagedTenants=true` kako biste vratili događaje iz vašeg najmodavca i svih najmodavaca koje on upravlja u jednom odgovoru. Svaki vraćeni zapis ima `tenantId` koji vam govori iz kojeg najmodavca potječe.

[inline-code-attrs-start title = 'Primjer cURL-a za AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Maksimum 10000. Zadano je 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Samo događaji koje je izvršio ovo korisničko ime. **/
    username?: string
    /** Samo događaji s ove IP adrese. **/
    ip?: string
    /** Samo događaji ovog tipa. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Samo događaji za ovaj resurs, npr. User ili Moderator. **/
    resourceName?: string
    /** Samo događaji čiji je zahvaćeni objekt ima ovaj ID. **/
    targetId?: string
    /** Podudaranje podstringa neosjetljivog na veličinu slova na oznaci zahvaćenog objekta. **/
    target?: string
    /** Također vrati događaje iz najmodavaca koje ovaj najmodavac upravlja. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Zapisnici! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---