[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Этот маршрут перемещает комментарии и страницы из одного `urlId` или домена в другой, так же как инструмент [Domain Migration tool](/guide-migrations.html#migrating-domains) в панели управления. За один запрос можно отправить до 100 миграций. Они выполняются в указанном порядке, поэтому переход от `a` к `b`, а затем от `b` к `c` работает.

Каждая миграция имеет `type`:

- `urlId` перемещает страницу, у которой `urlId` точно совпадает с `from`. `post-1` не совпадает с `post-10`.
- `domain` ищет/заменяет `from` внутри каждого `urlId` и `url`, и обновляет `domain` каждого комментария. Оба значения должны быть доменом, например `old.example.com`.

Комментарии, страница, реакции к странице, голоса, уведомления и подписки также перемещаются. Если страница уже существует по адресу `to`, две страницы объединяются.

Миграция выполняется как пакетная задача. Ответ содержит `batchJobId`. Делайте опрос [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id), чтобы отслеживать её прогресс. После завершения задачи она имеет результат для каждой миграции и `undoId`, который можно передать в [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) в течение 90 дней.

Запрос проверяется перед постановкой в очередь. Если какая‑либо миграция недействительна, весь запрос завершается ошибкой `422` и ничего не выполняется.

Постановка миграции в очередь не стоит ничего. Когда задача завершается, она стоит `1` кредит за каждые `1000` перемещённых комментариев, минимум `1`.

[inline-code-attrs-start title = 'Пример cURL миграций страниц'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запроса миграций страниц'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Выполняется в указанном порядке. Не более 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** Не более 1000 символов. Начальные и конечные пробелы удаляются. **/
    from: string
    /** Должно отличаться от "from". Не более 1000 символов. **/
    to: string
    /** "urlId" совпадает только с точным urlId. "domain" ищет/заменяет значение внутри каждого urlId и url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа миграций страниц'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Включается при ошибке. Для invalid-migration указывает индекс первой недействительной миграции. **/
    reason?: string
    /** Передайте в GET /api/v1/batch-jobs/:id для получения прогресса и результата. **/
    batchJobId?: string
}
[inline-code-end]

#### Примечания

- Задачи для одного арендатора выполняются по одной, начиная с самой старой. Вторая миграция ждёт завершения первой.
- `PATCH /api/v1/pages/:id` с новым `urlId` изменяет только страницу, а не её комментарии. Используйте этот маршрут для перемещения комментариев страницы.

---