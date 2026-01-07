[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת לבטל חסימה של משתמש שכתב תגובה נתונה. היא תומכת בביטול חסימה מתגובות שנכתבו על ידי משתמשי FastComments.com, משתמשי SSO ומשתמשי שוכר.

היא תומכת בפרמטר גוף `commentIdsToCheck` כדי לבדוק אם תגובות אחרות שעלולות להיות גלויות בלקוח צריכות להיחסם/להיפתח לאחר ביצוע פעולה זו.

הערות:

- קריאה זו חייבת תמיד להיעשות בהקשר של משתמש. המשתמש יכול להיות משתמש FastComments.com, משתמש SSO או משתמש שוכר.
- ה-`userId` בבקשה הוא המשתמש שמבצע את ביטול החסימה. לדוגמה: `משתמש א'` רוצה לבטל חסימה של `משתמש ב'`. העבר `userId=משתמש א'` ואת מזהה התגובה ש`משתמש ב'` כתב.
- תגובות אנונימיות לחלוטין (ללא מזהה משתמש, ללא אימייל) לא ניתנות לחסימה ושגיאה תוחזר.

[inline-code-attrs-start title = 'דוגמת cURL לביטול חסימת תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'דוגמת cURL לביטול חסימת תגובה אנונימית'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת ביטול חסימת תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת ביטול חסימת תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are still blocked. If false, you might want to un-hide the comments from the user so they don't have to refresh. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
