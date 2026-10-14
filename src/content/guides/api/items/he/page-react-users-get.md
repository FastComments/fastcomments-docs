[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

מחזיר את שמות המשתמשים שהוסיפו תגובה לדף, ממוינים באלפבית. מחפשים עד 100 תגובות, ומשתמשים אנונימיים אינם נכללים. בדפים הדורשים SSO, משתמשים שלא יכולים לצפות בדף מקבלים `403` עם הקוד `sso-required` או `page-forbidden`.

כדי גם לקבל תמונות פרופיל ו‑IDs של משתמשים, השתמש ב‑[`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) עם `includeUserInfo=public`.

[inline-code-attrs-start title = 'דוגמת cURL למשתמשי תגובות בדף'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת משתמשי תגובות בדף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** מזהה התגובה. **/
    id: string
    /** JSON מקודד URI של אובייקט ה‑SSO שלך. השאר ריק למשתמשים אנונימיים. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת משתמשי תגובות בדף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---