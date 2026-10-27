[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Този маршрут премества коментари и страници от един `urlId` или домейн към друг, същото като [инструмента за миграция на домейни](/guide-migrations.html#migrating-domains) в таблото. До 100 миграции могат да бъдат изпратени в една заявка. Те се изпълняват в зададения ред, така че `a` към `b`, последвано от `b` към `c`, работи.

Всяка миграция има `type`:

- `urlId` премества страницата, чийто `urlId` е точно `from`. `post-1` не съвпада с `post-10`.
- `domain` намира/заменя `from` във всеки `urlId` и `url` и актуализира `domain` на всеки коментар. И двете стойности трябва да бъдат домейн, като `old.example.com`.

Коментарите, страницата, реакциите към страницата, гласовете, известията и абонаментите се преместват. Ако страница вече съществува на `to`, двете страници се обединяват.

Миграцията се изпълнява като пакетна задача. Отговорът съдържа `batchJobId`. Извършвайте заявки [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id), за да следите прогреса. След като задачата завърши, тя има резултат за всяка миграция и `undoId`, който можете да предадете на [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) в рамките на 90 дни.

Заявката се валидира преди да бъде поставена в опашка. Ако някоя миграция е невалидна, цялата заявка се проваля с код `422` и нищо не се изпълнява.

Поставянето в опашка на миграция не струва нищо. Когато задачата завърши, се начислява `1` кредит за всеки `1000` мигрирани коментари, с минимум `1`.

[inline-code-attrs-start title = 'Пример cURL за миграции на страници'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за миграции на страници'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на отговор за миграции на страници'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

- Задачите за един наемател се изпълняват една по една, най-старите първо. Втора миграция изчаква първата да завърши.
- `PATCH /api/v1/pages/:id` с нов `urlId` променя само страницата, а не нейните коментари. Използвайте този маршрут, за да преместите коментарите на страница.