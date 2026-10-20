[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Ta pot potuje komentarje in strani z enega `urlId` ali domene na drugega, enako kot orodje [Domain Migration tool](/guide-migrations.html#migrating-domains) v nadzorni plošči. Na zahtevo je mogoče poslati največ 100 selitev. Izvajajo se v podanem vrstnem redu, zato `a` na `b`, sledeno z `b` na `c`, deluje.

Vsaka selitev ima `type`:

- `urlId` premakne stran, katere `urlId` je točno `from`. `post-1` se ne ujema z `post-10`.
- `domain` poišče/zamenja `from` znotraj vsakega `urlId` in `url` ter posodobi `domain` vsakega komentarja. Obe vrednosti morata biti domeni, npr. `old.example.com`.

Komentarji, stran, reakcije na stran, glasovi, obvestila in naročnine se vsi premaknejo. Če stran na `to` že obstaja, se dve strani združita.

Selitev se izvede kot paketno opravilo. Odgovor vsebuje `batchJobId`. Poizvedujte [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id), da spremljate napredek. Ko je opravilo končano, ima rezultat za vsako selitev in `undoId`, ki ga lahko pošljete na [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) v roku 90 dni.

Zahteva se preveri, preden se karkoli doda v čakalno vrsto. Če je katera koli selitev neveljavna, celotna zahteva ne uspe z napako `422` in se nič ne izvede.

V čakalno vrsto postavitev selitve ne stane nič. Ko opravilo konča, stane `1` kredit na `1000` premaknjenih komentarjev, z minimalno vrednostjo `1`.

[inline-code-attrs-start title = 'Primer cURL za selitev strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za selitev strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Run in the order given. At most 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** At most 1000 characters. Leading and trailing whitespace is removed. **/
    from: string
    /** Must be different from "from". At most 1000 characters. **/
    to: string
    /** "urlId" only matches the exact urlId. "domain" find/replaces the value inside every urlId and url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za selitev strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Included on failure. For invalid-migration, this names the index of the first invalid migration. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]

#### Notes

- Opravila za enega najemnika se izvajajo po eno, najprej najstarejše. Druga selitev čaka, da se prva konča.
- `PATCH /api/v1/pages/:id` z novim `urlId` spremeni le stran, ne njenih komentarjev. Uporabite to pot za premik komentarjev strani.