[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

מחזיר את שמות המשתמשים שהוסיפו תגובה לעמוד, ממוינים לפי סדר אלפביתי. מחפשים עד 100 תגובות, ומשתמשים אנונימיים אינם נכללים.

[inline-code-attrs-start title = 'דוגמת cURL למשתמשי תגובות בעמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת משתמשי תגובות בעמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** מזהה התגובה. **/
    id: string
    /** JSON מקודד ב‑URI של אובייקט ה‑SSO שלך. השמט עבור משתמשים אנונימיים. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת משתמשי תגובות בעמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כשל. **/
    code?: string
    /** נכלל במקרה של כשל. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]