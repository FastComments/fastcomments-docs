[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

API זה משתמש בעימוד, המסופק על ידי פרמטר השאילתה `skip`. TenantUsers מוחזרים בעמודים של `100`, מסודרים לפי `signUpDate`, `username` ו-`id`.

העלות מבוססת על מספר משתמשי השוכר שהוחזרו, בעלות של `1 קרדיט לכל 10` משתמשי שוכר שהוחזרו.

[inline-code-attrs-start title = 'דוגמת cURL ל-TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant users to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]
