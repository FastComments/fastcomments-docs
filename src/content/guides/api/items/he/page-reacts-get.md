[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

מחזיר את הספירה עבור כל תגובה בעמוד, ואת אילו תגובות המשתמש הנוכחי הוסיף.

העבר `includeUserInfo=public` כדי גם לקבל את המשתמשים שהגיבו, כערך אחד לכל תגובה עם מזהה התגובה שלה, newest first. משתמשים עם פעילות פרופיל פרטית ומשתמשים אנונימיים מושמטים, וניתן לחפש עד 100 תגובות. רק `public` מתקבל כאן; `include-private` מחזיר `422` וזמין מ-[`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) עם מפתח API. בעמודים הדורשים SSO, `includeUserInfo` מחזיר `403` למשתמשים שלא יכולים לצפות בעמוד.

[inline-code-attrs-start title = 'דוגמת cURL לתגובות עמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת תגובות עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** JSON מקודד ב‑URI של אובייקט SSO שלך. השמט עבור משתמשים אנונימיים. **/
    sso?: string
    /** הגדר ל‑'public' כדי לכלול את המשתמשים שהגיבו. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת תגובות עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** כלול במקרה של כשל. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** כלול במקרה של כשל. **/
    reason?: string
    /** ספירה לכל מזהה תגובה, לדוגמה {"heart": 12, "laugh": 3}. לא מוגדר כאשר לעמוד אין תגובות. **/
    counts?: Record<string, number>
    /** מזהי התגובות שהמשתמש המבצע את הבקשה הוסיף. לא מוגדר כאשר הוא לא הוסיף אף אחת. **/
    reactedIds?: string[]
    /** ערך אחד לכל תגובה, newest first. מוגדר רק כאשר נשלח includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** מזהה התגובה, לדוגמה "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]