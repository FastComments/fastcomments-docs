[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

API זה מחזיר שוכרים המנוהלים על ידי השוכר שלך.

עימוד מסופק על ידי פרמטר השאילתה `skip`. שוכרים מוחזרים בעמודים של `100`, מסודרים לפי `signUpDate`, ו-`id`.

העלות מבוססת על מספר השוכרים שהוחזרו, בעלות של `1 קרדיט לכל 10` שוכרים שהוחזרו.

[inline-code-attrs-start title = 'דוגמת cURL ל-Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

אתה יכול להגדיר פרמטרי `meta` על אובייקטי ה-`Tenant` ולחפש שוכרים תואמים. לדוגמה, עבור המפתח `someKey` וערך המטא `some-value`, אנחנו יכולים
לבנות אובייקט JSON עם זוג מפתח/ערך זה ואז לקודד אותו כ-URI כפרמטר שאילתה לסינון:

[inline-code-attrs-start title = 'דוגמת cURL לשאילתת Tenant לפי Meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenants to skip for pagination. **/
    skip?: number
    /** Filter by meta params. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]
