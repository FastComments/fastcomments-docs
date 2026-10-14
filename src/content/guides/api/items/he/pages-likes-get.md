[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

מחזיר את מספר הלייקים בעמוד, ובאופן אופציונלי את המשתמשים שלחצו לייק, מהחדש ביותר לישן. זו הגרסה בצד השרת של [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` controls which users are returned:

- `public` משאיר מחוץ למשתמשים שהפכו את פעילות הפרופיל שלהם לפרטית.
- `include-private` כולל אותם. השתמשו בזה רק כאשר הרשימה אינה מוצגת לציבור.

משתמשים אנונימיים אף פעם לא נכללים, וניתן לחפש עד 100 לייקים. דפים שאינם קיימים מחזירים `likeCount` של `0` ורשימת `users` ריקה.

[inline-code-attrs-start title = 'דוגמת cURL של לייקים בעמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת לייקים בעמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** השמט כדי לקבל רק את ספירת הלייקים. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת לייקים בעמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** נכלל במקרה של כשל. **/
    reason?: string
    likeCount?: number
    /** מהחדש ביותר לישן. מוגדר רק כאשר includeUserInfo נשלח. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### טיפ מועיל

זכרו לקודד ערכים ב-URI כמו `urlId`.