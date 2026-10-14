[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

מחזיר את הספירה עבור כל תגובה בעמוד, ובאופן אופציונלי את המשתמשים שהגיבו, כערך אחד לכל תגובה עם מזהה התגובה שלה, כשהכי חדשות ראשונות. זו הגרסה בצד השרת של [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` שולט באילו משתמשים יוחזרו:

- `public` משאיר מחוץ משתמשים שהפכו את פעילות הפרופיל שלהם לפרטית.
- `include-private` כולל אותם. השתמשו בזה רק כאשר הרשימה אינה מוצגת לציבור.

משתמשים אנונימיים אף פעם לא נכללים, וניתן לחפש עד 100 תגובות. דפים שאינם קיימים מחזירים ללא `counts` ורשימת `users` ריקה.

[inline-code-attrs-start title = 'דוגמת cURL לתגובות עמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת תגובות עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** הימנע כדי לקבל רק את הספירות. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת תגובות עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** נכלל במקרה של כשל. **/
    reason?: string
    /** ספירה לכל מזהה תגובה, לדוגמה {"heart": 12, "laugh": 3}. לא מוגדר כאשר לעמוד אין תגובות. **/
    counts?: Record<string, number>
    /** ערך אחד לכל תגובה, הכי חדשות ראשונות. מוגדר רק כאשר includeUserInfo נשלח. **/
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

#### טיפ מועיל

זכרו לקודד ערכים ב-URI כמו `urlId`.