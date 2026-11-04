[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

מחזירה את מספר הלייקים בעמוד, ובאופן אופציונלי את המשתמשים שעשו לייק, מהחדש ביותר לישן ביותר. זו הגרסה בצד השרת של [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

הפרמטר `includeUserInfo` קובע אילו משתמשים יוחזרו:

- `public` משמיט משתמשים שהגדירו את פעילות הפרופיל שלהם כפרטית.
- `include-private` כולל אותם. השתמשו באפשרות זו רק כאשר הרשימה אינה מוצגת לציבור.

משתמשים אנונימיים אינם נכללים מעולם, ומבוצעת בדיקה של עד 100 לייקים. עמודים שאינם קיימים מחזירים `likeCount` של `0` ורשימת `users` ריקה.

[inline-code-attrs-start title = 'דוגמת cURL ל-Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** השמיטו כדי לקבל רק את מספר הלייקים. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תשובה של Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כישלון. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** נכלל במקרה של כישלון. **/
    reason?: string
    likeCount?: number
    /** מהחדש ביותר לישן ביותר. מוגדר רק כאשר includeUserInfo מועבר. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### טיפ שימושי

זכרו לבצע URI Encode לערכים כמו `urlId`.