[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

מוסיף תגובה לעמוד כמשתמש הנוכחי. משתמש יכול להוסיף כל מזהה תגובה פעם אחת: הוספה חוזרת מצליחה עם הקוד `already-reacted` ולא משנה את הספירה. משתמש יכול להוסיף כמה תגובות שונות לאותו עמוד.

מזהי תגובות נבחרים על ידך ויכולים להיות עד 36 תווים. העמוד נוצר אם הוא עדיין לא קיים. העבר `title` כדי להגדיר או לעדכן את כותרת העמוד.

[inline-code-attrs-start title = 'דוגמת cURL לתגובת עמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת תגובת עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** מזהה התגובה, עד 36 תווים. **/
    id: string
    /** מגדיר את כותרת העמוד. **/
    title?: string
    /** JSON מקודד ב‑URI של אובייקט ה‑SSO שלך. השמט עבור משתמשים אנונימיים. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת תגובת עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' כאשר המשתמש כבר הוסיף תגובה זו. 'react-id-too-long' (HTTP 422) כאשר המזהה ארוך מ‑36 תווים. אחרת נכלל בכישלון. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** נכלל בכישלון. **/
    reason?: string
}
[inline-code-end]