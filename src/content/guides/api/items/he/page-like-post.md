[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

מחבב דף כמשתמש הנוכחי. כל משתמש יכול לאהוב דף פעם אחת: אהבה חוזרת מצליחה עם הקוד `already-liked` ולא משנה את הספירה.

הדף נוצר אם הוא עדיין לא קיים. העבר את `title` כדי להגדיר או לעדכן את כותרת הדף.

[inline-code-attrs-start title = 'דוגמת cURL לייק של דף'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת לייק של דף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** מגדיר את כותרת הדף. **/
    title?: string
    /** JSON מקודד ב-URI של אובייקט ה-SSO שלך. השמט עבור משתמשים אנונימיים. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת לייק של דף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' כאשר המשתמש כבר אהב את הדף. אחרת נכלל בכישלון. **/
    code?: 'already-liked' | string
    /** נכלל בכישלון. **/
    reason?: string
}
[inline-code-end]