[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

נתיב זה מספק את היכולת לשלוח קישור התחברות ל-`TenantUser` יחיד.

שימושי בעת יצירת משתמשים בכמות ללא צורך להנחות אותם כיצד להתחבר ל-FastComments.com. זה פשוט ישלח להם "קישור קסם" להתחברות שפג תוקף
לאחר `30 יום`.

ההגבלות הבאות קיימות לשליחת קישור התחברות ל-`TenantUser`:
- ה-`TenantUser` חייב כבר להתקיים.
- חייבת להיות לך גישה לנהל את ה-`Tenant` שאליו ה-`TenantUser` שייך.

אנחנו יכולים לשלוח קישור התחברות ל-`TenantUser` כדלקמן:

[inline-code-attrs-start title = 'דוגמת cURL לקישור התחברות TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

זה ישלח אימייל כמו `Bob ב-TenantName מזמין אותך להיות מנחה...`

[inline-code-attrs-start title = 'מבנה בקשת קישור התחברות TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת קישור התחברות TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
