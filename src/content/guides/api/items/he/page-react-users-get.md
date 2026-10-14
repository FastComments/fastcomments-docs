[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

מחזיר את שמות המשתמשים שהוסיפו תגובה לדף, ממוינים לפי סדר אלפביתי. מחפשים עד 100 תגובות, ומשתמשים אנונימיים ומשתמשים עם פעילות פרופיל פרטית אינם נכללים. בדפים הדורשים SSO, משתמשים שלא יכולים לצפות בדף מקבלים `403` עם הקוד `sso-required` או `page-forbidden`.

כדי גם לקבל תמונות פרופיל ומזהי משתמש, השתמש ב-[`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) עם `includeUserInfo=public`.

[inline-code-attrs-start title = 'דוגמת cURL למשתמשי תגובות לדף'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת משתמשי תגובות לדף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת משתמשי תגובות לדף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]