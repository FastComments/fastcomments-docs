[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Цей маршрут переміщує коментарі та сторінки з одного `urlId` або домену до іншого, так само, як інструмент [Domain Migration tool](/guide-migrations.html#migrating-domains) у панелі управління. За один запит можна надіслати до 100 міграцій. Вони виконуються у вказаному порядку, тому `a` до `b`, а потім `b` до `c` працює.

Кожна міграція має `type`:

- `urlId` переміщує сторінку, у якої `urlId` точно дорівнює `from`. `post-1` не збігається з `post-10`.
- `domain` знаходить/замінює `from` у кожному `urlId` та `url`, і оновлює `domain` кожного коментаря. Обидва значення мають бути доменом, наприклад `old.example.com`.

Коментарі, сторінка, реакції на сторінку, голоси, сповіщення та підписки також переміщуються. Якщо сторінка вже існує за `to`, дві сторінки об’єднуються.

Міграція виконується як пакетна задача. Відповідь містить `batchJobId`. Виконуйте опитування [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id), щоб стежити за її прогресом. Після завершення задачі вона має результат для кожної міграції та `undoId`, який можна передати до [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) протягом 90 днів.

Запит валідовано перед тим, як його поставити в чергу. Якщо будь‑яка міграція недійсна, весь запит завершується помилкою `422` і нічого не виконується.

Постановка міграції в чергу не коштує нічого. Після завершення задачі вартість становить `1` кредит за `1000` переміщених коментарів, мінімум `1`.

[inline-code-attrs-start title = 'Приклад cURL для міграції сторінок'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запиту міграції сторінок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура відповіді міграції сторінок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Примітки

- Завдання для одного орендаря виконуються по одному, спочатку найстаріші. Друга міграція чекає, доки перша завершиться.
- `PATCH /api/v1/pages/:id` з новим `urlId` змінює лише сторінку, а не її коментарі. Використовуйте цей маршрут, щоб перемістити коментарі сторінки.