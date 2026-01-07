[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

נתיב זה מספק הסרה של `Tenant` **וכל הנתונים המשויכים** (משתמשים, תגובות, וכו') לפי מזהה.

ההגבלות הבאות קיימות סביב הסרת שוכרים:

- השוכר חייב להיות שלך, או שוכר white labeled שאתה מנהל.
- פרמטר השאילתה `sure` חייב להיות מוגדר ל-`true`.

[inline-code-attrs-start title = 'דוגמת cURL להסרת שוכר'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת הסרת שוכר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת הסרת שוכר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
