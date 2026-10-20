[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs/:id'; creditsCost = 1; api-resource-header-end]

נתיב זה מחזיר את המצב, ההתקדמות ו‑, לאחר שהסתיים, את התוצאה של משימת batch אחת. השתמש בו כדי לעקוב אחרי משימה שהחלה על ידי [`POST /api/v1/pages/migrations`](#pages-migrations-post) או [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post).

משימה נחשבת להסתיימה כאשר ה‑`status` שלה הוא `success` או `failed`. כל קריאה עולה `1` קרדיט, ולכן עדיף לבצע polling כל כמה שניות במקום באופן רציף.

[inline-code-attrs-start title = 'דוגמת cURL של BatchJob לפי מזהה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs/my-batch-job-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת BatchJob לפי מזהה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת BatchJob לפי מזהה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenantid' | 'invalidtenantid' | 'invalidapikey' | 'missingapikey' | 'notfound' | 'internal'
    /** Included on failure. **/
    reason?: string
    batchJob?: BatchJob
}
[inline-code-end]

[inline-code-attrs-start title = 'דוגמה של משימת הגירת דף שהסתיימה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "batchJob": {
        "id": "my-batch-job-id",
        "type": "migrate-pages",
        "status": "success",
        "createdAt": "2026-10-19T16:00:00.000Z",
        "completedAt": "2026-10-19T16:00:12.000Z",
        "processedCount": 2,
        "totalCount": 2,
        "pageMigrationResults": [
            {"from": "old-post-id", "to": "new-post-id", "type": "urlId", "status": "success", "commentsMigrated": 42},
            {"from": "old.example.com", "to": "new.example.com", "type": "domain", "status": "success", "commentsMigrated": 1250}
        ],
        "undoId": "my-undo-id"
    }
}
[inline-code-end]