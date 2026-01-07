[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת לדווח על תגובה עבור משתמש ספציפי.

הערות:

- קריאה זו חייבת תמיד להיעשות בהקשר של משתמש. המשתמש יכול להיות משתמש FastComments.com, משתמש SSO או משתמש שוכר.
- אם מוגדר סף דיווח-להסתרה, התגובה תוסתר אוטומטית בזמן אמת לאחר שדווחה את מספר הפעמים המוגדר.
- לאחר שהיא מאושרת אוטומטית (מוסתרת) - התגובה יכולה להיות מאושרת מחדש רק על ידי מנהל או מנחה. ביטול הדיווח לא יאשר מחדש את התגובה.

[inline-code-attrs-start title = 'דוגמת cURL לדיווח על תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

לדיווח אנונימי, עלינו לציין `anonUserId`. זה יכול להיות מזהה שמייצג את ההפעלה האנונימית, או UUID אקראי.
זה מאפשר לנו לתמוך בדיווח וביטול דיווח על תגובות גם אם משתמש לא מחובר. בדרך זו, התגובה יכולה להיות מסומנת כמדווחת
כאשר תגובות נאחזות עם אותו `anonUserId`.

[inline-code-attrs-start title = 'דוגמת cURL לדיווח אנונימי על תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'מבנה בקשת דיווח על תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת דיווח על תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
    /** Was the comment un-approved (hidden) due to being flagged too many times? **/
    wasUnapproved?: boolean;
}
[inline-code-end]
