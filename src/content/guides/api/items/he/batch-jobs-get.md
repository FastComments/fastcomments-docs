[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

נתיב זה מחזיר את משימות האצווה של השוכר מכל סוג, החדשות ביותר תחילה, 100 לכל עמוד. השתמש ב-`type` וב-`status` כדי לצמצם את הרשימה, וב-`skip` כדי לדפדף.

[inline-code-attrs-start title = 'דוגמת cURL לרשימת BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת רשימת BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** רק משימות מהסוג הזה, לדוגמה migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** כמה משימות לדלג, לדפדוף. מספר שלם לא שלילי. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת רשימת BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** נכלל במקרה של כשל. **/
    reason?: string
    /** החדשות ביותר תחילה, לכל היותר 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]

---