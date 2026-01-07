[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

API זה משתמש בעימוד, המסופק על ידי הפרמטרים `skip`, `before` ו-`after`. AuditLogs מוחזרים בדפים של `1000`, ממוינים לפי `when` ו-`id`.

אחזור של כל `1000` יומנים עולה `10` קרדיטים.

כברירת מחדל, תקבל רשימה עם **הפריטים החדשים ביותר ראשונים**. בדרך זו, אתה יכול לתשאל החל מ-`skip=0`, לדפדף עד שתמצא את הרשומה האחרונה שצרכת.

לחלופין, אתה יכול למיין מהישן לחדש, ולדפדף עד שאין יותר רשומות.

ניתן לבצע מיון על ידי הגדרת `order` ל-`ASC` או `DESC`. ברירת המחדל היא `ASC`.

שאילתה לפי תאריך אפשרית דרך `before` ו-`after` כחותמות זמן עם מילישניות. `before` ו-`after` אינם כוללניים.

[inline-code-attrs-start title = 'דוגמת cURL ל-AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]
