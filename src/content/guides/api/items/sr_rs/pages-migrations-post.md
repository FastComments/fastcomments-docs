[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Овај рут помера коментаре и странице са једног `urlId` или домена на други, исто као [алат за миграцију домена](/guide-migrations.html#migrating-domains) у контролној табли. До 100 миграција може бити послато по захтеву. Они се извршавају редом у коме су наведени, тако да `a` у `b` затим `b` у `c` функционише.

Each migration has a `type`:

- `urlId` премешта страницу чији је `urlId` тачно `from`. `post-1` не одговара `post-10`.
- `domain` претражује/замењује `from` унутар сваког `urlId` и `url`, и ажурира `domain` сваког коментара. Обе вредности морају бити домен, нпр. `old.example.com`.

Коментари, страница, реакције на страницу, гласови, обавештења и претплате се све премештају. Ако страница већ постоји на `to`, две странице се спајају.

Миграција се извршава као пакетни посао. Одговор садржи `batchJobId`. Периодично позивајте [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) да пратите њен напредак. Када посао заврши, има резултат по миграцији, и `undoId` који можете проследити у [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) у року од 90 дана.

Захтев се валидира пре него што се стави у ред. Ако је нека миграција неважећа, цео захтев не успеје са `422` и ништа се не извршава.

Стављање миграције у ред не кошта ништа. Када посао заврши, кошта `1` кредит по `1000` мигрисаних коментара, са минимумом од `1`.

[inline-code-attrs-start title = 'Пример cURL захтева за миграцију страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура захтева за миграцију страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура одговора за миграцију страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Белешке

- Послови за једног тенанта се извршавају један по један, прво најстарији. Друга миграција чека да прва заврши.
- `PATCH /api/v1/pages/:id` са новим `urlId` мења само страницу, а не њене коментаре. Користите овај рут за премештање коментара странице.