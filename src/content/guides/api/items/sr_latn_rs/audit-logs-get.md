[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ovaj API koristi paginaciju, koju obezbeđuju parametri `skip`, `limit`, `before` i `after`. AuditLog‑ovi se vraćaju u stranicama od po `1000` po defaultu, do maksimalnog `limit` od `10000`, poređani po `when` i `id`. Stranice su velike jer se ovaj endpoint obično koristi za izvoz istorije, a ne za interaktivno listanje.

Svaki `100` vraćenih logova košta `1` kredit.

Podrazumevano, dobićete listu sa **najnovijim stavkama prvo**. Na ovaj način možete da poll‑ujete počevši od `skip=0`, paginirajući dok ne nađete poslednji zapis koji ste potrošili.

Alternativno, možete sortirati od najstarijih ka najnovijim i paginirati dok ne ostane više zapisa.

Sortiranje se može izvršiti postavljanjem `order` na `ASC` ili `DESC`. Podrazumevano je `DESC`.

Upit po datumu je moguć putem `before` i `after` kao vremenskih oznaka u milisekundama. `before` i `after` NISU inkluzivni i bilo koji se može koristiti samostalno.

## Pronalaženje šta se desilo osobi

Svaki događaj beleži ko ga je izvršio (`username`, `userId`, `ip`) i, odvojeno, na čemu je izvršen. `targetLabel` je čitljiva oznaka za taj objekat, na primer `jsmith (jsmith@example.com)`, a `targetId` je njegov ID. Koristite `target` za podudaranje podstringa neosetljivog na veličinu slova na oznaci kada znate ime ili email osobe, ali ne i njen ID.

Brisanja beleže oznaku u trenutku događaja, tako da uklonjeni korisnik ili moderator i dalje mogu biti identifikovani nakon što osnovni zapis više ne postoji.

## Upravljani tenant‑i

Ako vaš tenant upravlja drugim tenant‑ima, postavite `includeManagedTenants=true` da biste vratili događaje iz vašeg tenant‑a i svakog tenant‑a koji on upravlja u jednom odgovoru. `tenantId` svakog vraćenog loga pokazuje iz kojeg tenant‑a potiče.

[inline-code-attrs-start title = 'AuditLog cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Struktura Zahteva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Maksimum 10000. Podrazumevano 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Samo događaji izvršeni od strane ovog korisničkog imena. **/
    username?: string
    /** Samo događaji sa ove IP adrese. **/
    ip?: string
    /** Samo događaji ovog tipa. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Samo događaji za ovaj resurs, npr. User ili Moderator. **/
    resourceName?: string
    /** Samo događaji čiji pogođeni objekat ima ovaj ID. **/
    targetId?: string
    /** Podudaranje podstringa neosetljivog na veličinu slova na oznaci pogođenog objekta. **/
    target?: string
    /** Takođe vrati događaje iz tenant‑a koje ovaj tenant upravlja. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Struktura Odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Logovi! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---