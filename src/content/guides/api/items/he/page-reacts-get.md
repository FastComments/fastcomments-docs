[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

מחזיר את הספירה לכל תגובה בעמוד, ואת אילו תגובות הוסיף המשתמש הנוכחי.

[inline-code-attrs-start title = 'דוגמת cURL של תגובות עמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת תגובות עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** JSON מקודד ב‑URI של אובייקט ה‑SSO שלך. השמט למשתמשים אנונימיים. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת תגובות עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** כלול במקרה של כשל. **/
    code?: string
    /** כלול במקרה של כשל. **/
    reason?: string
    /** ספירה לכל מזהה תגובה, לדוגמה {"heart": 12, "laugh": 3}. לא מוגדר כאשר לעמוד אין תגובות. **/
    counts?: Record<string, number>
    /** מזהי התגובות שהמשתמש שביצע את הבקשה הוסיף. לא מוגדר כאשר הוא לא הוסיף אף אחת. **/
    reactedIds?: string[]
}
[inline-code-end]