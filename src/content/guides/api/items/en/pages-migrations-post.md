[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

This route moves comments and pages from one `urlId` or domain to another, the same as the [Domain Migration tool](/guide-migrations.html#migrating-domains) in the dashboard. Up to 100 migrations can be sent per request. They run in the order given, so `a` to `b` followed by `b` to `c` works.

Each migration has a `type`:

- `urlId` moves the page whose `urlId` is exactly `from`. `post-1` does not match `post-10`.
- `domain` find/replaces `from` inside every `urlId` and `url`, and updates the `domain` of each comment. Both values must be a domain, like `old.example.com`.

Comments, the page, page reacts, votes, notifications and subscriptions all move. If a page already exists at `to`, the two pages are merged.

The migration runs as a batch job. The response contains a `batchJobId`. Poll [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) to follow its progress. Once the job is done it has a result per migration, and an `undoId` you can pass to [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) within 90 days.

The request is validated before anything is queued. If any migration is invalid, the whole request fails with a `422` and nothing runs.

Queueing a migration costs nothing. When the job finishes it costs `1` credit per `1000` comments migrated, with a minimum of `1`.

[inline-code-attrs-start title = 'Page Migrations cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Page Migrations Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Page Migrations Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

- Jobs for one tenant run one at a time, oldest first. A second migration waits for the first to finish.
- `PATCH /api/v1/pages/:id` with a new `urlId` only changes the page, not its comments. Use this route to move a page's comments.
