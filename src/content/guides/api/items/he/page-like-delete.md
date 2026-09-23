[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

מסיר את הלייק של המשתמש הנוכחי מדף. אם המשתמש לא אהב את הדף, הבקשה מצליחה עם הקוד `not-liked` ולא משנה את הספירה.

[inline-code-attrs-start title = 'דוגמת cURL לביטול לייק בדף'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת ביטול לייק בדף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** JSON מקודד ב-URI של אובייקט ה-SSO שלך. השמט עבור משתמשים אנונימיים. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת ביטול לייק בדף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' כאשר המשתמש לא אהב את הדף. אחרת כלול בכישלון. **/
    code?: 'not-liked' | string
    /** כלול בכישלון. **/
    reason?: string
}
[inline-code-end]

---