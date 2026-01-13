[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

API זה משתמש בעימוד, המסופק על ידי פרמטר השאילתה `skip`. TenantPackages מוחזרים בעמודים של `100`, מסודרים לפי `createdAt` ו-`id`.

העלות מבוססת על מספר חבילות השוכר שהוחזרו, בעלות של `1 קרדיט לכל 10` חבילות שוכר שהוחזרו.

[inline-code-attrs-start title = 'דוגמת cURL ל-TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant packages to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]
