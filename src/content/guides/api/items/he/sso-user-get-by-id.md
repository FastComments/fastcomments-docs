[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users/by-id/:id'; creditsCost = 1; api-resource-header-end]

נתיב זה מחזיר משתמש SSO יחיד לפי המזהה שלו.

[inline-code-attrs-start title = 'דוגמת cURL ל-SSOUser לפי מזהה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users/by-id/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserByIdResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
    user: SSOUser
}
[inline-code-end]
