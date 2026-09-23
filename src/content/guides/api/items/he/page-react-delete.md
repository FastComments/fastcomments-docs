[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

מסיר אחת מהתגובות של המשתמש הנוכחי מדף. אם המשתמש לא הוסיף את התגובה הזו, הבקשה מצליחה עם הקוד `no-react` ולא משנה את הספירה.

[inline-code-attrs-start title = 'דוגמת cURL למחיקת תגובת דף'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת מחיקת תגובת דף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** מזהה התגובה. **/
    id: string
    /** JSON מקודד ב-URI של אובייקט ה-SSO שלך. השמט למשתמשים אנונימיים. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת מחיקת תגובת דף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' כאשר המשתמש לא הוסיף את התגובה הזו. אחרת נכלל בכישלון. **/
    code?: 'no-react' | string
    /** נכלל בכישלון. **/
    reason?: string
}
[inline-code-end]

---